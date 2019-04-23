use std::sync::{Arc, Mutex};

use ui::{ClickHandler, HasClickHandlers, HasText, MainMenuView, HandlerRegistration};

use event::{ListenerRegistration, EventBus, EventListener, EnchantronEvent, StartGame};


pub struct MainMenuPresenter<V : MainMenuView> {
  view: V,
  handler_registrations: Mutex<Vec<Box<HandlerRegistration>>>,
  listener_registrations: Mutex<Vec<ListenerRegistration>>,
  event_bus: Arc<EventBus>
}

impl <V: MainMenuView> EventListener<StartGame> for MainMenuPresenter<V> {
  fn on_event(&self, _: &StartGame) {
    self.view.transition_to_game_view()
  }
}

impl <V: MainMenuView> MainMenuPresenter<V> {

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

  fn bind(self) -> Arc<MainMenuPresenter<V>> {
    let copied_event_bus = self.event_bus.clone();

    self.add_handler_registration(Box::new(self.view
        .get_start_new_game_button()
        .add_click_handler(create_click_handler!({
            copied_event_bus.post(StartGame{new: true})
        }))));

    let result = Arc::new(self);

    result.add_listener_registration(
        result.event_bus.register(EnchantronEvent::StartGame, &result));

    result.view.get_start_new_game_button().set_text(
        "Start New Game".to_string());

    result
  }

  pub fn new(view: V, event_bus: Arc<EventBus>)
      -> Arc<MainMenuPresenter<V>> {
    let result = MainMenuPresenter {
      view: view,
      handler_registrations: Mutex::new(Vec::new()),
      listener_registrations: Mutex::new(Vec::new()),
      event_bus: event_bus,
    };

    result.bind()
  }

}


impl <V: MainMenuView> Drop for MainMenuPresenter<V> {
  fn drop(&mut self) {
    println!("Dropping Main Menu Presenter")
  }
}