use std::sync::{Arc, Weak};
use tokio::sync::{Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::view_types::ViewTypes;

use crate::event::*;

use crate::view::BaseView;

use crate::model::{Point, Rect, Size};

use crate::native::{RuntimeResources, SystemView};

use crate::ui::{
    DragHandler, DragState, GameDisplayState, HandlerRegistration,
    HasDragHandlers, HasLayoutHandlers, HasMagnifyHandlers, HasMutableLocation,
    HasMutableScale, HasViewport, LayoutHandler, MagnifyHandler, Sprite,
    SpriteSource, SpriteSourceWrapper, ViewportInfo,
};

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
    runtime_resources: Arc<RuntimeResources<T::SystemView>>,
    listener_registrations: Mutex<Vec<ListenerRegistration>>,
    handler_registrations: Mutex<Vec<Box<dyn HandlerRegistration>>>,

    weak_self: RwLock<Option<Box<Weak<GamePresenter<T>>>>>,

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
    async fn with_display_state(
        &self,
        action: impl FnOnce(&GameDisplayState<T>),
    ) {
        if let Some(display_state) = self.display_state.read().await.as_ref() {
            action(display_state)
        }
    }

    ///
    /// Run an action with a write lock on the game display state
    ///
    async fn with_display_state_mut(
        &self,
        action: impl FnOnce(&mut GameDisplayState<T>),
    ) {
        let ref mut display_state_lock = self.display_state.write().await;

        let display_state = display_state_lock.as_mut().unwrap_or_else(|| {
            error!("No Game State created yet");
            panic!("No Game State created yet");
        });

        action(display_state);
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

            let viewport_info = display_state.change_scale_additive(
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

    async fn on_drag(&self, drag_event: Drag) {
        match drag_event.state {
            DragEventType::Start => self.on_drag_start(drag_event).await,
            DragEventType::Move => self.on_drag_move(drag_event).await,
            DragEventType::End => self.on_drag_end(drag_event).await,
        }
    }

    async fn on_drag_start(&self, drag_start_event: Drag) {
        let Drag {
            state: _,
            global_point: drag_point,
            local_point: _,
        } = drag_start_event;

        debug!("Drag started {:?}", drag_point);

        self.with_display_state_mut(|display_state| {
            display_state.drag_state =
                Option::Some(DragState::new(drag_point.clone()));
        })
        .await;
    }

    async fn on_drag_move(&self, drag_move_event: Drag) {
        let Drag {
            state: _,
            global_point:
                Point {
                    x: drag_x,
                    y: drag_y,
                },
            local_point: _,
        } = drag_move_event;

        debug!("Drag moved ({}, {})", drag_x, drag_y);

        self.with_display_state_mut(|display_state| {
            let scale = display_state.get_viewport_scale();

            let position_shift = if let Some(mut drag_state) =
                display_state.drag_state.as_mut()
            {
                let screen_coord_delta = Point::new(
                    drag_state.last_drag_point.x - drag_x,
                    drag_state.last_drag_point.y - drag_y,
                );

                drag_state.last_drag_point.x = drag_x;
                drag_state.last_drag_point.y = drag_y;

                Point::new(
                    screen_coord_delta.x * scale,
                    screen_coord_delta.y * scale,
                )
            } else {
                error!("Invalid drag state found");
                panic!("Invalid drag state found");
            };

            let new_viewport_info =
                display_state.move_viewport_by(position_shift);

            self.fire_viewport_change_event(new_viewport_info);

            let new_position_ref = &new_viewport_info.viewport_rect.top_left;

            self.view
                .get_viewport()
                .set_location(new_position_ref.x, new_position_ref.y);
        })
        .await;
    }

    async fn on_drag_end(&self, _: Drag) {
        debug!("Drag ended");
        self.with_display_state_mut(|ds| ds.drag_state = Option::None)
            .await;
    }

    /// Initialize the display state with the initial game state
    async fn initialize_game_state(&self) {
        let sprite_source_self = self.weak_self().await;

        let mut display_state: GameDisplayState<T> = GameDisplayState::new(
            self.event_bus.clone(),
            SpriteSourceWrapper::new(move || {
                sprite_source_self
                    .upgrade()
                    .map(|p| p.create_sprite())
                    .unwrap_or_else(|| {
                        error!("Failed to create sprite");
                        panic!("Failed to create sprite");
                    })
            }),
            self.runtime_resources.clone(),
            self.system_view.clone(),
        )
        .await;

        display_state.set_character_sprite(self.view.create_sprite());

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

        let result_drag_start = self.weak_self().await;
        let result_drag_move = self.weak_self().await;
        let result_drag_end = self.weak_self().await;

        self.add_handler_registration(Box::new(self.view.add_drag_handler(
            create_drag_handler!(
                on_drag_start(wx, wy, lx, ly) {
                    result_drag_start.upgrade().map(|p| {
                        p.event_bus.post(Drag {
                            state: DragEventType::Start,
                            global_point: Point { x: wx, y: wy },
                            local_point: Point { x: lx, y: ly }
                        });
                    });
                },
                on_drag_move(wx, wy, lx, ly) {
                    result_drag_move.upgrade().map(|p| {
                        p.event_bus.post(Drag {
                            state: DragEventType::Move,
                            global_point: Point { x: wx, y: wy },
                            local_point: Point { x: lx, y: ly }
                        });
                    });
                },
                on_drag_end(wx, wy, lx, ly) {
                    result_drag_end.upgrade().map(|p| {
                        p.event_bus.post(Drag {
                            state: DragEventType::End,
                            global_point: Point { x: wx, y: wy },
                            local_point: Point { x: lx, y: ly }
                        });
                    });
                }
            ),
        )))
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

        handle_event!(Drag => self.on_drag);

        handle_event!(Magnify => self.on_magnify);
    }

    pub fn create_sprite(&self) -> T::Sprite {
        self.view.create_sprite()
    }

    pub async fn new(
        view: T::GameView,
        event_bus: EventBus,
        runtime_resources: Arc<RuntimeResources<T::SystemView>>,
        system_view: Arc<T::SystemView>,
    ) -> Arc<GamePresenter<T>> {
        view.initialize_pre_bind();

        let raw_result = GamePresenter {
            view,
            event_bus,
            runtime_resources,
            system_view,
            listener_registrations: Mutex::new(Vec::new()),
            handler_registrations: Mutex::new(Vec::new()),

            weak_self: RwLock::new(Default::default()),

            display_state: RwLock::new(Default::default()),
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
