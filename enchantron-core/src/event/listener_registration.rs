pub struct ListenerRegistration {
  deregister: Box<Fn() + 'static>
}

impl ListenerRegistration {
  pub fn new(deregister: Box<Fn() + 'static>) -> ListenerRegistration {
    ListenerRegistration {
      deregister: deregister
    }
  }
}

impl Drop for ListenerRegistration {
  fn drop(&mut self) {
    (self.deregister)()
  }
}