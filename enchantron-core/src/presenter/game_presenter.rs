use ui::{GameView};
use event::{EventBus, ListenerRegistration};
use std::sync::{Mutex,Arc};


pub struct GamePresenter<V: GameView> {
  view: V,
  event_bus: Arc<EventBus>,
  handler_registrations: Mutex<Vec<ListenerRegistration>>
}

impl <V: GameView> GamePresenter<V> {

  fn bind(self) -> Arc<GamePresenter<V>> {

    let result = Arc::new(self);

    result
  }

  pub fn new(view: V, event_bus: Arc<EventBus>) -> Arc<GamePresenter<V>> {
    let result = GamePresenter{
      view: view,
      event_bus: event_bus,
      handler_registrations: Mutex::new(Vec::new())
    };

    result.bind()
  }

}

impl <V: GameView> Drop for GamePresenter<V> {
  fn drop(&mut self) {
    println!("Dropping Game Presenter")
  }
}