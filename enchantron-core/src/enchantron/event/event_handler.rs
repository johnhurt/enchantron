pub trait EventHandler<E> : 'static {
  fn on_event(&self, event: &E);
}