use std::ops::DerefMut;
use std::sync::{Arc, Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard, Weak};

use crate::view_types::ViewTypes;

use crate::event::{
    EnchantronEvent, EventBus, EventListener, HasListenerRegistrations, Layout,
    ListenerRegistration, ViewportChange,
};

use crate::model::{Point, Rect, Size};

use crate::native::RuntimeResources;

use crate::ui::{
    DragHandler, DragState, GameDisplayState, HandlerRegistration,
    HasDragHandlers, HasLayoutHandlers, HasMagnifyHandlers, HasMutableLocation,
    HasMutableScale, HasMutableVisibility, HasViewport, LayoutHandler,
    MagnifyHandler, Sprite, SpriteSource, SpriteSourceWrapper,
};

pub struct GamePresenter<T>
where
    T: ViewTypes,
{
    view: T::GameView,
    event_bus: EventBus<EnchantronEvent>,
    runtime_resources: Arc<RuntimeResources<T::SystemView>>,
    listener_registrations: Mutex<Vec<ListenerRegistration>>,
    handler_registrations: Mutex<Vec<Box<dyn HandlerRegistration>>>,

    weak_self: RwLock<Option<Box<Weak<GamePresenter<T>>>>>,

    display_state: RwLock<Option<GameDisplayState<T>>>,
}

impl<T> HasListenerRegistrations for GamePresenter<T>
where
    T: ViewTypes,
{
    fn add_listener_registration(
        &self,
        listener_registration: ListenerRegistration,
    ) {
        if let Ok(mut locked_list) = self.listener_registrations.lock() {
            info!("Adding listener registration to Game Presenter");
            locked_list.push(listener_registration);
        } else {
            error!("Failed to add listener registration to Game Presenter");
        }
    }
}

impl<T> EventListener<EnchantronEvent, Layout> for GamePresenter<T>
where
    T: ViewTypes,
{
    fn on_event(&self, event: &Layout) {
        info!("Game view resized to : {}, {}", event.width, event.height);

        let new_size = Size::new(event.width as f64, event.height as f64);

        self.with_display_state_mut(|display_state| {
            let viewport_info = display_state.layout(new_size);

            self.fire_viewport_change_event(
                viewport_info.viewport_rect.clone(),
            );

            self.view
                .get_viewport()
                .set_location_point(&viewport_info.viewport_rect.top_left);
        });
    }
}

impl<T> GamePresenter<T>
where
    T: ViewTypes,
{
    /// Get a weak arc pointer to this presenter or panic if none has been
    /// created yet
    fn weak_self(&self) -> Weak<GamePresenter<T>> {
        let ref weak_self_lock = self.weak_self.read().unwrap_or_else(|err| {
            error!("Failed to get read lock on weak self pointer: {:?}", err);
            panic!("Failed to get a read lock on weak self pointer");
        });

        weak_self_lock
            .as_ref()
            .map(Box::as_ref)
            .map(Clone::clone)
            .unwrap_or_else(|| {
                error!("No weak self pointer created yet");
                panic!("No weak self pointer created yet");
            })
    }

    ///
    /// Run an action with a read lock on the game display state
    ///
    fn with_display_state(&self, action: impl FnOnce(&GameDisplayState<T>)) {
        let ref display_state_lock =
            self.display_state.read().unwrap_or_else(|err| {
                error!("Failed to get read lock on display state: {:?}", err);
                panic!("Failed to get a read lock on the display state");
            });

        let display_state = display_state_lock.as_ref().unwrap_or_else(|| {
            error!("No Game State created yet");
            panic!("No Game State created yet");
        });

        action(display_state);
    }

    ///
    /// Run an action with a write lock on the game display state
    ///
    fn with_display_state_mut(
        &self,
        action: impl FnOnce(&mut GameDisplayState<T>),
    ) {
        let ref mut display_state_lock =
            self.display_state.write().unwrap_or_else(|err| {
                error!("Failed to get write lock on display state: {:?}", err);
                panic!("Failed to get a write lock on the display state");
            });

        let display_state = display_state_lock.as_mut().unwrap_or_else(|| {
            error!("No Game State created yet");
            panic!("No Game State created yet");
        });

        action(display_state);
    }

    fn add_handler_registration(&self, hr: Box<dyn HandlerRegistration>) {
        if let Ok(mut locked_list) = self.handler_registrations.lock() {
            locked_list.push(hr);
        }
    }

    /// Fire a viewport change event to the event bus
    fn fire_viewport_change_event(&self, viewport_rect: Rect) {
        self.event_bus.post(ViewportChange {
            new_viewport_rect: viewport_rect,
        });
    }

    fn on_magnify(
        &self,
        scale_change_additive: f64,
        zoom_center_x: f64,
        zoom_center_y: f64,
    ) {
        debug!("Scale changing by {}", scale_change_additive);

        self.with_display_state_mut(|display_state| {
            let magnify_center_screen_point =
                Point::new(zoom_center_x, zoom_center_y);

            let viewport_info = display_state.change_scale_additive(
                scale_change_additive,
                magnify_center_screen_point,
            );

            self.fire_viewport_change_event(
                viewport_info.viewport_rect.clone(),
            );
            self.view.get_viewport().set_scale_and_location_point(
                viewport_info.viewport_scale,
                &viewport_info.viewport_rect.top_left,
            );
        });
    }

    fn on_drag_start(&self, drag_point: &Point) {
        debug!("Drag started {:?}", drag_point);

        self.with_display_state_mut(|display_state| {
            display_state.drag_state =
                Option::Some(DragState::new(drag_point.clone()));
        });
    }

    fn on_drag_move(&self, drag_x: f64, drag_y: f64) {
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

            self.fire_viewport_change_event(
                new_viewport_info.viewport_rect.clone(),
            );

            let new_position_ref = &new_viewport_info.viewport_rect.top_left;

            self.view
                .get_viewport()
                .set_location(new_position_ref.x, new_position_ref.y);
        });
    }

    fn on_drag_end(&self) {
        debug!("Drag ended");
        self.with_display_state_mut(|ds| ds.drag_state = Option::None);
    }

    /// Initialize the display state with the initial game state
    fn initialize_game_state(&self) {
        let sprite_source_self = self.weak_self();

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
        );

        display_state.set_character_sprite(self.view.create_sprite());

        let mut display_state_opt =
            self.display_state.write().unwrap_or_else(|err| {
                error!("Failed to get write lock on display state: {:?}", err);
                panic!("Failed to get a write lock on the display state");
            });

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
        )));

        let result_drag_start = self.weak_self();
        let result_drag_move = self.weak_self();
        let result_drag_end = self.weak_self();

        self.add_handler_registration(Box::new(self.view.add_drag_handler(
            create_drag_handler!(
                on_drag_start(wx, wy, _lx, _ly) {
                    result_drag_start.upgrade()
                        .map(|p| p.on_drag_start(&Point { x: wx, y: wy }));
                },
                on_drag_move(wx, wy, _lx, _ly) {
                    result_drag_move.upgrade()
                        .map(|p| p.on_drag_move(wx, wy));
                },
                on_drag_end(_wx, _wy, _lx, _ly) {
                    result_drag_end.upgrade()
                        .map(|p| p.on_drag_end());
                }
            ),
        )));

        let result_for_magnify = self.weak_self();

        self.add_handler_registration(Box::new(self.view.add_magnify_handler(
            create_magnify_handler!(
                on_magnify(scale_change_additive, center_x, center_y) {
                    result_for_magnify.upgrade()
                        .map(|p| {
                            p.on_magnify(
                                scale_change_additive,
                                center_x,
                                center_y)
                        });
                }
            ),
        )));

        self.event_bus.register(Layout::default(), self.weak_self()).await;
    }

    pub fn create_sprite(&self) -> T::Sprite {
        self.view.create_sprite()
    }

    pub async fn new(
        view: T::GameView,
        event_bus: EventBus<EnchantronEvent>,
        runtime_resources: Arc<RuntimeResources<T::SystemView>>,
    ) -> Arc<GamePresenter<T>> {
        let raw_result = GamePresenter {
            view: view,
            event_bus: event_bus,
            runtime_resources: runtime_resources,
            listener_registrations: Mutex::new(Vec::new()),
            handler_registrations: Mutex::new(Vec::new()),

            weak_self: Default::default(),

            display_state: Default::default(),
        };

        let result = Arc::new(raw_result);

        {
            let weak_self = Arc::downgrade(&result);
            let mut weak_self_opt =
                result.weak_self.write().unwrap_or_else(|e| {
                    error!(
                        "Failed to get write lock on weak self pointer: {:?}",
                        e
                    );
                    panic!("Failed to get write lock on weak self pointer");
                });

            *weak_self_opt = Some(Box::new(weak_self));
        }

        GamePresenter::bind(&result).await;
        result.initialize_game_state();

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
