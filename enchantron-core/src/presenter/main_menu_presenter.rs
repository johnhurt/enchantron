use std::sync::{Arc, Mutex};

use crate::ui::{
    ClickHandler, HandlerRegistration, HasClickHandlers, HasText, MainMenuView,
};

use crate::event::{
    EnchantronEvent, EventBus, EventListener, HasListenerRegistrations,
    ListenerRegistration, StartGame,
};

pub struct MainMenuPresenter<V: MainMenuView> {
    view: V,
    handler_registrations: Mutex<Vec<Box<dyn HandlerRegistration>>>,
    listener_registrations: Mutex<Vec<ListenerRegistration>>,
    event_bus: EventBus<EnchantronEvent>,
}

impl<V: MainMenuView> EventListener<EnchantronEvent, StartGame>
    for MainMenuPresenter<V>
{
    fn on_event(&self, _: &StartGame) {
        self.view.transition_to_game_view()
    }
}

impl<V> HasListenerRegistrations for MainMenuPresenter<V>
where
    V: MainMenuView,
{
    fn add_listener_registration(
        &self,
        listener_registration: ListenerRegistration,
    ) {
        if let Ok(mut locked_list) = self.listener_registrations.lock() {
            info!("Adding listener registration to loading presenter");
            locked_list.push(listener_registration);
        } else {
            error!("Failed to add listener registration");
        }
    }
}

impl<V: MainMenuView> MainMenuPresenter<V> {
    fn add_listener_registration(&self, lr: ListenerRegistration) {
        if let Ok(mut locked_list) = self.listener_registrations.lock() {
            locked_list.push(lr);
        }
    }

    fn add_handler_registration(&self, hr: Box<dyn HandlerRegistration>) {
        if let Ok(mut locked_list) = self.handler_registrations.lock() {
            locked_list.push(hr);
        }
    }

    fn bind(self) -> Arc<MainMenuPresenter<V>> {
        let copied_event_bus = self.event_bus.clone();

        self.add_handler_registration(Box::new(
            self.view.get_start_new_game_button().add_click_handler(
                create_click_handler!({
                    copied_event_bus.post(StartGame { new: true })
                }),
            ),
        ));

        let result = Arc::new(self);

        result.event_bus.register(StartGame::default(), Arc::downgrade(&result));

        result
            .view
            .get_start_new_game_button()
            .set_text("New Game".to_string());

        result
    }

    pub fn new(
        view: V,
        event_bus: EventBus<EnchantronEvent>,
    ) -> Arc<MainMenuPresenter<V>> {
        let result = MainMenuPresenter {
            view: view,
            handler_registrations: Mutex::new(Vec::new()),
            listener_registrations: Mutex::new(Vec::new()),
            event_bus: event_bus,
        };

        result.bind()
    }
}

impl<V: MainMenuView> Drop for MainMenuPresenter<V> {
    fn drop(&mut self) {
        info!("Dropping Main Menu Presenter")
    }
}
