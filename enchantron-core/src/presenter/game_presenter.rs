use std::sync::{Arc, Mutex, RwLock, RwLockReadGuard};

use crate::view_types::ViewTypes;

use crate::event::{
    EnchantronEvent, EventBus, EventListener, Layout, ListenerRegistration,
};

use crate::model::{Point, Rect, Size};

use crate::native::{HasIntSize, RuntimeResources, SystemView};

use crate::ui::{
    DragHandler, GameDisplayState, GameView, HandlerRegistration,
    HasMutableLocation, HasMutableSize, HasMutableVisibility, LayoutHandler,
    Sprite, SpriteSource, HasLayoutHandlers, HasViewport
};

pub struct GamePresenter<T>
where
    T: ViewTypes
{
    view: T::GameView,
    event_bus: EventBus<EnchantronEvent>,
    runtime_resources: Arc<RuntimeResources<T::SystemView>>,
    listener_registrations: Mutex<Vec<ListenerRegistration>>,
    handler_registrations: Mutex<Vec<Box<HandlerRegistration>>>,

    display_state: RwLock<GameDisplayState<T::Sprite>>,
}

impl<T> EventListener<EnchantronEvent, Layout> for GamePresenter<T>
where
    T: ViewTypes
{
    fn on_event(&self, event: &Layout) {
        info!("Game view resized to : {}, {}", event.width, event.height);

        let display_state = self.get_display_state();
        let sprite = &display_state.grass;

        sprite.set_location(
            event.width as f64 / 2. - 32.,
            event.height as f64 / 2. - 32.,
        );
    }
}

impl<T> GamePresenter<T>
where
    T: ViewTypes
{
    ///
    /// Get a read-lock on the game display state
    ///
    fn get_display_state(&self) -> RwLockReadGuard<GameDisplayState<T::Sprite>> {
        self.display_state.read().unwrap_or_else(|err| {
            error!("Failed to get read lock on display state: {:?}", err);
            panic!("Failed to get a read lock on the display state");
        })
    }

    fn add_listener_registration(&self, lr: ListenerRegistration) {
        if let Ok(mut locked_list) = self.listener_registrations.lock() {
            locked_list.push(lr);
        }
    }

    fn add_handler_registration(&self, hr: Box<HandlerRegistration>) {
        if let Ok(mut locked_list) = self.handler_registrations.lock() {
            locked_list.push(hr);
        }
    }

    /// Initialize the display state with the initial game state
    fn initialize_game_state(this: Arc<GamePresenter<T>>) {
        let sprite = &this.get_display_state().grass;
        sprite.set_texture(this.runtime_resources.textures().overworld.grass());
        sprite.set_visible(true);
        sprite.set_size(64., 64.);
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

        result.add_listener_registration(
            result.event_bus.register(Layout::default(), &result),
        );

        result
    }

    pub fn new(
        view: T::GameView,
        event_bus: EventBus<EnchantronEvent>,
        runtime_resources: Arc<RuntimeResources<T::SystemView>>,
    ) -> Arc<GamePresenter<T>> {
        let display_state = GameDisplayState::new(&view.get_viewport());

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
    T: ViewTypes
{
    fn drop(&mut self) {
        info!("Dropping Game Presenter")
    }
}
