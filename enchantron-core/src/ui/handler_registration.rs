
pub trait HandlerRegistration : 'static {
  fn deregister(&self);
}