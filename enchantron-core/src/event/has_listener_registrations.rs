use super::ListenerRegistration;
use std::future::Future;

pub trait HasListenerRegistrations: 'static + Send {
    fn add_listener_registration(
        &self,
        listener_registration: ListenerRegistration,
    ) -> Future<Output = ()>;
}
