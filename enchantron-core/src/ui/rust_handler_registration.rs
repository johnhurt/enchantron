use super::HandlerRegistration;
use std::sync::atomic::{AtomicU64, Ordering};

pub struct RustHandlerRegistration {
    deregister_fn: Option<Box<dyn FnOnce() + Send + Sync + 'static>>,
}

impl RustHandlerRegistration {
    pub fn new(
        deregister: impl FnOnce() + Send + Sync + 'static,
    ) -> RustHandlerRegistration {
        RustHandlerRegistration {
            deregister_fn: Some(Box::new(deregister)),
        }
    }
}

impl HandlerRegistration for RustHandlerRegistration {
    fn deregister(&self) {}
}

impl Drop for RustHandlerRegistration {
    fn drop(&mut self) {
        self.deregister_fn.take().map(|dereg| (dereg)());
    }
}
