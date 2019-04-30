pub trait EventListener<E>: 'static {
    fn on_event(&self, event: &E);
}
