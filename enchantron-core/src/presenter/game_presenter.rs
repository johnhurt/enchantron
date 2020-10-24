use super::TerrainPresenter;
use crate::application_context::{Ao, NUM_CPUS};
use crate::event::*;
use crate::game::{Gor, SavedGame, Services};
use crate::model::{Point, Size};
use crate::native::RuntimeResources;
use crate::ui::{
    DragTrackerEvent::*, GameDisplayState, HandlerRegistration,
    HasLayoutHandlers, HasMagnifyHandlers, HasMultiDragHandlers,
    HasMutableLocation, HasMutableScale, HasViewport, LayoutHandler,
    MagnifyHandler, MultiDragHandler, SpriteSource,
};
use crate::view::BaseView;
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
    view: T::GameView,
    event_bus: EventBus,
    runtime_resources: Ao<RuntimeResources<T>>,
    system_view: Ao<T::SystemView>,
    _handler_registrations: Vec<Box<dyn HandlerRegistration>>,

    display_state: GameDisplayState,

    droppers: Vec<Box<dyn FnOnce() + Send>>,
}

impl<T> GamePresenter<T>
where
    T: ViewTypes,
{
    async fn on_layout(&mut self, event: Layout) {
        info!("Game view resized to : {}, {}", event.width, event.height);

        let new_size = Size::new(event.width as f64, event.height as f64);

        let viewport_info = self.display_state.layout(new_size);

        self.event_bus.post(ViewportChange::new(*viewport_info));

        self.view
            .get_viewport()
            .set_location_point(&viewport_info.viewport_rect.top_left);
    }

    async fn on_magnify(&mut self, magnify_event: Magnify) {
        let Magnify {
            scale_change_additive,
            global_center:
                Point {
                    x: zoom_center_x,
                    y: zoom_center_y,
                },
        } = magnify_event;

        debug!("Scale changing by {}", scale_change_additive);

        let magnify_center_screen_point =
            Point::new(zoom_center_x, zoom_center_y);

        let viewport_info = self
            .display_state
            .change_scale_additive_around_center_point(
                scale_change_additive,
                magnify_center_screen_point,
            );

        self.event_bus.post(ViewportChange::new(*viewport_info));

        self.view.get_viewport().set_scale_and_location_point(
            viewport_info.viewport_scale,
            &viewport_info.viewport_rect.top_left,
        );
    }

    async fn on_drag(&mut self, drag_event: DragEvent) {
        let drag_tracker_event =
            self.display_state.drag_tracker.on_drag_event(drag_event);

        match drag_tracker_event {
            Some(Move(drag_move)) => self.on_drag_move(drag_move).await,
            Some(MoveAndScale(drag_move, scale)) => {
                self.on_drag_move_and_scale(drag_move, scale).await
            }
            _ => (),
        }
    }

    async fn on_drag_move(&mut self, drag_move: Point) {
        let scale = self.display_state.get_viewport_scale();

        let position_shift = drag_move * scale;

        let new_viewport_info =
            self.display_state.move_viewport_by(position_shift);

        self.event_bus.post(ViewportChange::new(*new_viewport_info));

        let new_position_ref = &new_viewport_info.viewport_rect.top_left;

        self.view
            .get_viewport()
            .set_location_point(new_position_ref);
    }

    async fn on_drag_move_and_scale(
        &mut self,
        drag_move: Point,
        new_scale: f64,
    ) {
        let new_viewport_info = self
            .display_state
            .change_scale_and_move(new_scale, drag_move);

        self.event_bus.post(ViewportChange::new(*new_viewport_info));

        let new_position_ref = &new_viewport_info.viewport_rect.top_left;

        self.view.get_viewport().set_scale_and_location_point(
            new_viewport_info.viewport_scale,
            new_position_ref,
        );
    }

    /// Initialize the display state with the initial game state
    fn initialize_game_state() -> GameDisplayState {
        GameDisplayState::new()
    }

    fn bind_ui_events(
        view: &T::GameView,
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

        result.push(Box::new(view.add_multi_drag_handler(
            MultiDragHandler::new(move |drag_event| {
                copied_event_bus.post::<UI>(drag_event.into())
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
        view: T::GameView,
        event_bus: EventBus,
        runtime_resources: Ao<RuntimeResources<T>>,
        system_view: Ao<T::SystemView>,
    ) {
        view.initialize_pre_bind();

        let saved_game = SavedGame::new(Default::default());

        let boxed_runtime = Box::new(
            Builder::new()
                .threaded_scheduler()
                .core_threads(NUM_CPUS.checked_sub(1).unwrap_or(1)) // one for os
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

        let display_state = Self::initialize_game_state();

        services
            .run(
                entity_sprite_group,
                runtime_resources.clone(),
                run_bundles.into_iter(),
            )
            .await;

        let game_runtime = services.runtime();

        let mut presenter = GamePresenter {
            view,
            event_bus: event_bus.clone(),
            runtime_resources,
            system_view,
            _handler_registrations,
            display_state,
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
                UIEvent::DragEvent { event } => presenter.on_drag(event).await,
                UIEvent::Layout { event } => presenter.on_layout(event).await,
                UIEvent::Magnify { event } => presenter.on_magnify(event).await,
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
