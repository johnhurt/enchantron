use ui::{GameView};
use event::{EventBus, ListenerRegistration};
use std::sync::{Mutex,Arc};


pub struct GamePresenter<V: GameView> {
  view: V,
  event_bus: Arc<EventBus>,
  handler_registrations: Mutex<Vec<ListenerRegistration>>
}

impl <V: GameView> GamePresenter<V> {

  pub fn new(view: V, event_bus: Arc<EventBus>) -> Arc<GamePresenter<V>> {
    Arc::new(GamePresenter{
      view: view,
      event_bus: event_bus,
      handler_registrations: Mutex::new(Vec::new())
    })
  }

}