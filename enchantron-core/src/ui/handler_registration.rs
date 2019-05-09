pub trait HandlerRegistration: 'static + Send + Sync {
    fn deregister(&self);
}
