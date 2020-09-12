use super::PlayerPresenter;
use crate::application_context::NUM_CPUS;
use crate::event::*;
use crate::game::{EntityType, SavedGame, Services};
use crate::model::{Point, Rect, Size};
use crate::native::{RuntimeResources, SystemView};
use crate::ui::{
    DragEventType, DragState, DragTrackerEvent::*, GameDisplayState,
    HandlerRegistration, HasLayoutHandlers, HasMagnifyHandlers,
    HasMultiDragHandlers, HasMutableLocation, HasMutableScale, HasViewport,
    LayoutHandler, MagnifyHandler, MultiDragHandler, Sprite, SpriteSource,
    ViewportInfo,
};
use crate::view::{BaseView, PlayerViewImpl};
use crate::view_types::ViewTypes;
use std::sync::{Arc, Weak};
use std::time::Duration;
use tokio::runtime::{Builder, Runtime};
use tokio::sync::{Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};

use tokio::stream::StreamExt;

macro_rules! handle_event {
    ($event_type:ident => $self_id:ident.$method_name:ident) => {
        let weak_self = $self_id.weak_self().await;
        let mut event_stream = $self_id.register_event::<$event_type>().await;

        $self_id.event_bus.spawn(async move {
            while let Some(event) = event_stream.next().await {
                if let Some(presenter) = weak_self.upgrade() {
                    presenter.$method_name(event).await;
                } else {
                    break;
                }
            }
        });
    };
}

pub struct GamePresenter<T>
where
    T: ViewTypes,
{
    view: T::GameView,
    event_bus: EventBus,
    system_view: Arc<T::SystemView>,
    runtime_resources: Arc<RuntimeResources<T>>,
    listener_registrations: Mutex<Vec<ListenerRegistration>>,
    handler_registrations: Mutex<Vec<Box<dyn HandlerRegistration>>>,

    weak_self: RwLock<Option<Box<Weak<GamePresenter<T>>>>>,

    game_runtime: Runtime,

    display_state: RwLock<Option<GameDisplayState<T>>>,
}

impl<T> GamePresenter<T>
where
    T: ViewTypes,
{
    /// Get a weak arc pointer to this presenter or panic if none has been
    /// created yet
    async fn weak_self(&self) -> Weak<GamePresenter<T>> {
        let ref weak_self_lock = self.weak_self.read().await;

        weak_self_lock
            .as_ref()
            .map(Box::as_ref)
            .map(Clone::clone)
            .unwrap_or_else(|| {
                error!("No weak self pointer created yet");
                panic!("No weak self pointer created yet");
            })
    }

    async fn register_event<E: Event>(&self) -> impl StreamExt<Item = E> {
        let (listener_registration, event_stream) =
            self.event_bus.register::<E>();

        self.listener_registrations
            .lock()
            .await
            .push(listener_registration);

        event_stream
    }

    ///
    /// Run an action with a read lock on the game display state
    ///
    async fn with_display_state<R>(
        &self,
        action: impl FnOnce(&GameDisplayState<T>) -> R,
    ) -> R {
        if let Some(display_state) = self.display_state.read().await.as_ref() {
            action(display_state)
        } else {
            panic!("Failed to get display state");
        }
    }

    ///
    /// Run an action with a write lock on the game display state
    ///
    async fn with_display_state_mut<R>(
        &self,
        action: impl FnOnce(&mut GameDisplayState<T>) -> R,
    ) -> R {
        let ref mut display_state_lock = self.display_state.write().await;

        let display_state = display_state_lock.as_mut().unwrap_or_else(|| {
            error!("No Game State created yet");
            panic!("No Game State created yet");
        });

        action(display_state)
    }

    async fn add_handler_registration(&self, hr: Box<dyn HandlerRegistration>) {
        self.handler_registrations.lock().await.push(hr);
    }

    /// Fire a viewport change event to the event bus
    fn fire_viewport_change_event(&self, viewport_info: &ViewportInfo) {
        self.event_bus.post(ViewportChange {
            new_viewport: viewport_info.clone(),
        });
    }

    async fn on_layout(&self, event: Layout) {
        info!("Game view resized to : {}, {}", event.width, event.height);

        let new_size = Size::new(event.width as f64, event.height as f64);

        self.with_display_state_mut(|display_state| {
            let viewport_info = display_state.layout(new_size);

            self.fire_viewport_change_event(viewport_info);

            self.view
                .get_viewport()
                .set_location_point(&viewport_info.viewport_rect.top_left);
        })
        .await;
    }

    async fn on_magnify(&self, magnify_event: Magnify) {
        let Magnify {
            scale_change_additive,
            global_center:
                Point {
                    x: zoom_center_x,
                    y: zoom_center_y,
                },
        } = magnify_event;

        debug!("Scale changing by {}", scale_change_additive);

        self.with_display_state_mut(|display_state| {
            let magnify_center_screen_point =
                Point::new(zoom_center_x, zoom_center_y);

            let viewport_info = display_state
                .change_scale_additive_around_centerpoint(
                    scale_change_additive,
                    magnify_center_screen_point,
                );

            self.fire_viewport_change_event(viewport_info);
            self.view.get_viewport().set_scale_and_location_point(
                viewport_info.viewport_scale,
                &viewport_info.viewport_rect.top_left,
            );
        })
        .await;
    }

    async fn on_drag(&self, drag_event: DragEvent) {
        let drag_tracker_event = self
            .with_display_state_mut(|display_state| {
                display_state.drag_tracker.on_drag_event(drag_event)
            })
            .await;

        match drag_tracker_event {
            Some(Move(drag_move)) => self.on_drag_move(drag_move).await,
            Some(MoveAndScale(drag_move, scale)) => {
                self.on_drag_move_and_scale(drag_move, scale).await
            }
            _ => (),
        }
    }

    async fn on_drag_move(&self, drag_move: Point) {
        self.with_display_state_mut(|display_state| {
            let scale = display_state.get_viewport_scale();

            let position_shift = drag_move * scale;

            let new_viewport_info =
                display_state.move_viewport_by(position_shift);

            self.fire_viewport_change_event(new_viewport_info);

            let new_position_ref = &new_viewport_info.viewport_rect.top_left;

            self.view
                .get_viewport()
                .set_location_point(new_position_ref);
        })
        .await;
    }

    async fn on_drag_move_and_scale(&self, drag_move: Point, new_scale: f64) {
        self.with_display_state_mut(|display_state| {
            let new_viewport_info =
                display_state.change_scale_and_move(new_scale, drag_move);

            self.fire_viewport_change_event(new_viewport_info);

            let new_position_ref = &new_viewport_info.viewport_rect.top_left;

            self.view.get_viewport().set_scale_and_location_point(
                new_viewport_info.viewport_scale,
                new_position_ref,
            );
        })
        .await;
    }

    /// Initialize the display state with the initial game state
    async fn initialize_game_state(&self) {
        let mut display_state: GameDisplayState<T> = GameDisplayState::new(
            self.event_bus.clone(),
            &self.view,
            self.runtime_resources.clone(),
            self.system_view.clone(),
        )
        .await;

        let mut display_state_opt = self.display_state.write().await;

        *display_state_opt = Some(display_state);
    }

    async fn bind(&self) {
        let copied_event_bus = self.event_bus.clone();

        self.add_handler_registration(Box::new(self.view.add_layout_handler(
            create_layout_handler!(|w, h| {
                copied_event_bus.post(Layout {
                    width: w,
                    height: h,
                })
            }),
        )))
        .await;

        let copied_event_bus = self.event_bus.clone();

        self.add_handler_registration(Box::new(
            self.view.add_multi_drag_handler(MultiDragHandler::new(
                move |drag_event| copied_event_bus.post(drag_event),
            )),
        ))
        .await;

        let result_for_magnify = self.weak_self().await;

        self.add_handler_registration(Box::new(self.view.add_magnify_handler(
            create_magnify_handler!(
                on_magnify(scale_change_additive, center_x, center_y) {
                    result_for_magnify.upgrade().map(|p| {
                        p.event_bus.post(Magnify {
                            scale_change_additive,
                            global_center: Point { x: center_x, y: center_y }
                        });
                    });
                }
            ),
        )))
        .await;

        handle_event!(Layout => self.on_layout);

        handle_event!(DragEvent => self.on_drag);

        handle_event!(Magnify => self.on_magnify);
    }

    pub async fn new(
        view: T::GameView,
        event_bus: EventBus,
        runtime_resources: Arc<RuntimeResources<T>>,
        system_view: Arc<T::SystemView>,
    ) -> Arc<GamePresenter<T>> {
        view.initialize_pre_bind();

        let saved_game = SavedGame::new(Default::default());

        let runtime = Builder::new()
            .threaded_scheduler()
            .core_threads(NUM_CPUS.checked_sub(1).unwrap_or(1)) // one for os
            .enable_time()
            .pausable_time(
                true,
                Duration::from_millis(saved_game.elapsed_millis),
            )
            .build()
            .unwrap_or_else(|e| {
                error!("Failed to create tokio runtime, {:?}", e);
                panic!("Failed to create tokio runtime");
            });

        let runtime_handle = runtime.handle().clone();

        let raw_result = GamePresenter {
            view,
            event_bus: event_bus.clone(),
            runtime_resources,
            system_view,
            listener_registrations: Mutex::new(Vec::new()),
            handler_registrations: Mutex::new(Vec::new()),

            weak_self: RwLock::new(Default::default()),
            display_state: RwLock::new(Default::default()),

            game_runtime: runtime,
        };

        let result: Arc<GamePresenter<T>> = Arc::new(raw_result);

        {
            let weak_self = Arc::downgrade(&result);
            let mut weak_self_opt = result.weak_self.write().await;

            *weak_self_opt = Some(Box::new(weak_self));
        }

        result.initialize_game_state().await;

        GamePresenter::bind(&result).await;

        result.view.initialize_post_bind(Box::new(result.clone()));

        let entity_sprite_group = Arc::new(result.view.create_group());
        let runtime_resources = result.runtime_resources.clone();

        let (services, run_bundles) =
            Services::new(runtime_handle.clone(), saved_game);
        let time = services.time();

        event_bus.spawn(async move { runtime_handle.resume() });

        result
    }
}

impl<T> Drop for GamePresenter<T>
where
    T: ViewTypes,
{
    fn drop(&mut self) {
        info!("Dropping Game Presenter")
    }
}
