pub struct ListenerRegistration {
    deregister: Option<Box<dyn FnOnce() + Sync + Send + 'static>>,
}

impl ListenerRegistration {
    pub fn new(
        deregister: Box<dyn FnOnce() + Sync + Send + 'static>,
    ) -> ListenerRegistration {
        ListenerRegistration {
            deregister: Some(deregister),
        }
    }
}

impl Drop for ListenerRegistration {
    fn drop(&mut self) {
        (self.deregister.take().unwrap())()
    }
}
