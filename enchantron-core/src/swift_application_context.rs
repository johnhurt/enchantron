use std::sync::Arc;
use event::EventBus;
use ::MainMenuView;


pub struct ApplicationContext {
  event_bus: Arc<EventBus>
}

impl ApplicationContext {
  pub fn new() -> ApplicationContext {
    ApplicationContext {
      event_bus: EventBus::new()
    }
  }


}