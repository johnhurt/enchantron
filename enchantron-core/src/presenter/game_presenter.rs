use std::sync::{Arc, Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::view_types::ViewTypes;

use crate::event::{
    EnchantronEvent, EventBus, EventListener, HasListenerRegistrations, Layout,
    ListenerRegistration,
};

use crate::model::{Point, Rect, Size};

use crate::native::RuntimeResources;

use crate::ui::{
    DragHandler, DragState, GameDisplayState, HandlerRegistration,
    HasDragHandlers, HasLayoutHandlers, HasMagnifyHandlers, HasMutableLocation,
    HasMutableScale, HasMutableVisibility, HasViewport, LayoutHandler,
    MagnifyHandler, Sprite,
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

    display_state: RwLock<GameDisplayState<T::Sprite>>,
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

        let mut display_state = self.get_display_state_mut();
        display_state.layout(new_size);

        self.view.get_viewport().set_location_point(
            &display_state
                .viewport_rect
                .as_ref()
                .map(|rect| &rect.top_left)
                .expect("Blah"),
        );
    }
}

impl<T> GamePresenter<T>
where
    T: ViewTypes,
{
    ///
    /// Get a read-lock on the game display state
    ///
    fn get_display_state(
        &self,
    ) -> RwLockReadGuard<GameDisplayState<T::Sprite>> {
        self.display_state.read().unwrap_or_else(|err| {
            error!("Failed to get read lock on display state: {:?}", err);
            panic!("Failed to get a read lock on the display state");
        })
    }

    ///
    /// Get a write-lock on the game display state
    ///
    fn get_display_state_mut(
        &self,
    ) -> RwLockWriteGuard<GameDisplayState<T::Sprite>> {
        self.display_state.write().unwrap_or_else(|err| {
            error!("Failed to get write lock on display state: {:?}", err);
            panic!("Failed to get a write lock on the display state");
        })
    }

    fn add_handler_registration(&self, hr: Box<dyn HandlerRegistration>) {
        if let Ok(mut locked_list) = self.handler_registrations.lock() {
            locked_list.push(hr);
        }
    }

    fn on_magnify(&self, scale_change_additive: f64) {
        debug!("Scale changing by {}", scale_change_additive);

        let mut display_state = self.get_display_state_mut();

        display_state.change_scale_additive(scale_change_additive);

        self.view
            .get_viewport()
            .set_scale(display_state.viewport_scale);
    }

    fn on_drag_start(&self, drag_point: &Point) {
        debug!("Drag started {:?}", drag_point);

        let mut display_state = self.get_display_state_mut();

        display_state.drag_state = Option::Some(DragState::new(
            drag_point.clone(),
            display_state
                .viewport_rect
                .clone()
                .unwrap_or_else(|| Rect::default()),
        ));
    }

    fn on_drag_move(&self, drag_x: f64, drag_y: f64) {
        debug!("Drag moved ({}, {})", drag_x, drag_y);

        let mut display_state = self.get_display_state_mut();

        let new_top_left =
            if let Some(drag_state) = display_state.drag_state.as_ref() {
                Point::new(
                    drag_state.start_viewport_rect.top_left.x - drag_x
                        + drag_state.start_point.x,
                    drag_state.start_viewport_rect.top_left.y - drag_y
                        + drag_state.start_point.y,
                )
            } else {
                error!("Invalid drag state found");
                panic!("Invalid drag state found");
            };

        self.view
            .get_viewport()
            .set_location(new_top_left.x, new_top_left.y);

        if let Some(ref mut viewport_rect) = display_state.viewport_rect {
            viewport_rect.top_left = new_top_left;
        }
    }

    fn on_drag_end(&self) {
        debug!("Drag ended");
        self.get_display_state_mut().drag_state = Option::None;
    }

    /// Initialize the display state with the initial game state
    fn initialize_game_state(this: Arc<GamePresenter<T>>) {
        let sprite = &this.get_display_state().grass;
        sprite.set_texture(this.runtime_resources.textures().overworld.grass());
        sprite.set_visible(true);
        //sprite.set_size(64., 64.);
    }

    fn bind(self) -> Arc<GamePresenter<T>> {
        let copied_event_bus = self.event_bus.clone();

        self.add_handler_registration(Box::new(self.view.add_layout_handler(
            create_layout_handler!(|w, h| {
                copied_event_bus.post(Layout {
                    width: w,
                    height: h,
                })
            }),
        )));

        let result = Arc::new(self);
        let result_drag_start = Arc::downgrade(&result);
        let result_drag_move = result_drag_start.clone();
        let result_drag_end = result_drag_start.clone();

        result.add_handler_registration(Box::new(
            result.view.add_drag_handler(create_drag_handler!(
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
            )),
        ));

        let result_for_magnify = Arc::downgrade(&result);

        result.add_handler_registration(Box::new(
            result.view.add_magnify_handler(create_magnify_handler!(
                on_magnify(scale_change_additive) {
                    result_for_magnify.upgrade()
                        .map(|p| p.on_magnify(scale_change_additive));
                }
            )),
        ));

        result.event_bus.register(Layout::default(), &result);

        result
    }

    pub fn new(
        view: T::GameView,
        event_bus: EventBus<EnchantronEvent>,
        runtime_resources: Arc<RuntimeResources<T::SystemView>>,
    ) -> Arc<GamePresenter<T>> {
        let display_state = GameDisplayState::new(&view);

        let result = GamePresenter {
            view: view,
            event_bus: event_bus,
            runtime_resources: runtime_resources,
            listener_registrations: Mutex::new(Vec::new()),
            handler_registrations: Mutex::new(Vec::new()),

            display_state: RwLock::new(display_state),
        };

        let arc_result = result.bind();

        GamePresenter::initialize_game_state(arc_result.clone());

        arc_result
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
