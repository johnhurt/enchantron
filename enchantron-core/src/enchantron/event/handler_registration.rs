pub struct HandlerRegistration {
  deregister: Box<Fn() + 'static>
}

impl HandlerRegistration {
  pub fn new(deregister: Box<Fn() + 'static>) -> HandlerRegistration {
    HandlerRegistration {
      deregister: deregister
    }
  }
}

impl Drop for HandlerRegistration {
  fn drop(&mut self) {
    (self.deregister)()
  }
}