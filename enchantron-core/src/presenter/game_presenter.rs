use super::{FocusedEntityPresenter, TerrainPresenter, ViewportPresenter};
use crate::application_context::{Ao, NUM_CPUS};
use crate::event::*;
use crate::game::{Gor, SavedGame, Services};
use crate::model::{Point, Size};
use crate::native::RuntimeResources;
use crate::ui::{
    HandlerRegistration, HasLayoutHandlers, HasMagnifyHandlers,
    HasMultiTouchHandlers, HasViewport, LayoutHandler, MagnifyHandler,
    MultiTouchHandler, SpriteSource, TouchEvent, TouchTracker,
};
use crate::view::{GameView, GameViewImpl, NativeView};
use crate::view_types::ViewTypes;
use futures::future::join_all;
use futures::pin_mut;
use std::time::Duration;
use tokio::runtime::Builder;
use tokio::select;
use tokio::stream::StreamExt;

pub struct GamePresenter<T>
where
    T: ViewTypes,
{
    view: GameViewImpl<T>,
    event_bus: EventBus,
    runtime_resources: Ao<RuntimeResources<T>>,
    system_view: Ao<T::SystemView>,

    touch_tracker: TouchTracker,
    viewport_presenter: ViewportPresenter<T>,
    focused_entity_presenter: FocusedEntityPresenter,

    droppers: Vec<Box<dyn FnOnce() + Send>>,
}

impl<T> GamePresenter<T>
where
    T: ViewTypes,
{
    fn on_layout(&mut self, event: Layout) {
        info!("Game view resized to : {}, {}", event.width, event.height);

        self.viewport_presenter.on_layout(&event);
    }

    fn on_magnify(&mut self, magnify_event: Magnify) {
        self.viewport_presenter.on_magnify(&magnify_event);
    }

    async fn on_touch(&mut self, raw_touch_event: RawTouchEvent) {
        let touch_event = self.touch_tracker.to_touch_event(
            &raw_touch_event,
            &self.viewport_presenter.viewport_info,
        );

        let touch_event_opt = self
            .focused_entity_presenter
            .on_touch_event(touch_event)
            .await;

        if let Some(touch_event) = touch_event_opt {
            self.viewport_presenter.on_touch_event(&touch_event);
        }
    }

    fn bind_ui_events(
        view: &GameViewImpl<T>,
        event_bus: EventBus,
    ) -> Vec<Box<dyn HandlerRegistration>> {
        let copied_event_bus = event_bus.clone();

        let mut result: Vec<Box<dyn HandlerRegistration>> = Vec::new();

        result.push(Box::new(view.add_layout_handler(create_layout_handler!(
            |w, h| {
                copied_event_bus.post::<UI>(
                    Layout {
                        width: w,
                        height: h,
                    }
                    .into(),
                )
            }
        ))));

        let copied_event_bus = event_bus.clone();

        result.push(Box::new(view.add_multi_touch_handler(
            MultiTouchHandler::new(move |touch_event| {
                copied_event_bus.post::<UI>(touch_event.into())
            }),
        )));

        let copied_event_bus = event_bus.clone();

        result.push(Box::new(view.add_magnify_handler(
            create_magnify_handler!(
                on_magnify(scale_change_additive, center_x, center_y) {
                    copied_event_bus.post::<UI>(Magnify {
                            scale_change_additive,
                            global_center: Point { x: center_x, y: center_y }
                    }.into());
                }
            ),
        )));

        result
    }

    fn create_sub_presenters(&mut self) -> (TerrainPresenter<T>) {
        let terrain_presenter = TerrainPresenter::new(
            self.event_bus.clone(),
            &self.view,
            self.runtime_resources.clone(),
            self.system_view.clone(),
        );

        (terrain_presenter)
    }

    pub async fn run(
        raw_view: T::NativeView,
        event_bus: EventBus,
        runtime_resources: Ao<RuntimeResources<T>>,
        system_view: Ao<T::SystemView>,
    ) {
        raw_view.initialize_pre_bind();
        let view = GameViewImpl::new(raw_view);
        let saved_game = SavedGame::new(Default::default());

        let boxed_runtime = Box::new(
            Builder::new_multi_thread()
                .thread_name("GameThread")
                .worker_threads(NUM_CPUS.checked_sub(1).unwrap_or(1)) // one for os
                .enable_time()
                .pausable_time(
                    true,
                    Duration::from_millis(saved_game.elapsed_millis),
                )
                .build()
                .unwrap_or_else(|e| {
                    panic!("Failed to create game runtime, {:?}", e);
                }),
        );

        let (services, run_bundles, mut droppers) =
            Services::new(boxed_runtime, saved_game);

        let boxed_entity_sprite_group = Box::new(view.create_group());
        let entity_sprite_group = Gor::new(&boxed_entity_sprite_group);

        droppers.push(Box::new(move || drop(boxed_entity_sprite_group)));

        let end_event = event_bus.register_for_one::<StopGameRequested>();
        let (_listener_reg, ui_stream) = event_bus.register::<UI>();

        let _handler_registrations =
            Self::bind_ui_events(&view, event_bus.clone());

        services
            .run(
                entity_sprite_group,
                runtime_resources.clone(),
                run_bundles.into_iter(),
            )
            .await;

        let game_runtime = services.runtime();

        let viewport_presenter =
            ViewportPresenter::new(view.get_viewport(), event_bus.clone());

        let focused_entity_presenter = FocusedEntityPresenter::new(services);

        let mut presenter = GamePresenter {
            view,
            event_bus: event_bus.clone(),
            runtime_resources,
            system_view,
            touch_tracker: Default::default(),
            viewport_presenter,
            focused_entity_presenter,
            droppers,
        };

        let (terrain_presenter) = presenter.create_sub_presenters();

        let sub_presenters_future =
            join_all(vec![event_bus.spawn(terrain_presenter.run())]);

        event_bus
            .register_for_one::<TerrainPresenterStarted>()
            .await;

        presenter.view.initialize_post_bind(Box::new(
            event_bus.post_on_drop(StopGameRequested::new()),
        ));

        event_bus.spawn_blocking(move || game_runtime.resume());

        pin_mut!(ui_stream);
        pin_mut!(end_event);

        // Main ui handler loop
        while let Some(UI { event: ui_event }) = select! {
            _ = &mut end_event => None,
            ui_event_opt = ui_stream.next() => ui_event_opt
        } {
            match ui_event {
                UIEvent::RawTouchEvent { event } => {
                    presenter.on_touch(event).await
                }
                UIEvent::Layout { event } => presenter.on_layout(event),
                UIEvent::Magnify { event } => presenter.on_magnify(event),
            }
        }

        sub_presenters_future.await;

        event_bus.post(GameStopped::new());
    }
}

impl<T> Drop for GamePresenter<T>
where
    T: ViewTypes,
{
    fn drop(&mut self) {
        info!("Dropping Game Presenter");

        // Run the droppers in the stored order so that the runtime is dropped
        // first. This is what ensures that the whole Gor system is safe
        for dropper in self.droppers.drain(..) {
            dropper();
        }
    }
}
