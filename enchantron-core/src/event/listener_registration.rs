pub struct ListenerRegistration {
    deregister: Box<dyn Fn() + Sync + Send + 'static>,
}

impl ListenerRegistration {
    pub fn new(
        deregister: Box<dyn Fn() + Sync + Send + 'static>,
    ) -> ListenerRegistration {
        ListenerRegistration {
            deregister: deregister,
        }
    }
}

impl Drop for ListenerRegistration {
    fn drop(&mut self) {
        (self.deregister)()
    }
}
