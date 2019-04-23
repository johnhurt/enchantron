use std::sync::Arc;

use event::EventBus;

use ui::LoadingView;

pub struct LoadingPresenter<V: LoadingView> {
  view: V,
  event_bus: Arc<EventBus>
}

impl <V : LoadingView> LoadingPresenter<V> {
  pub fn new(view: V, event_bus: Arc<EventBus>) -> Arc<LoadingPresenter<V>> {
    Arc::new(LoadingPresenter {
      view: view,
      event_bus: event_bus
    })
  }
}