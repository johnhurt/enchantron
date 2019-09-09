use super::ListenerRegistration;

pub trait HasListenerRegistrations: 'static + Send {
    fn add_listener_registration(
        &self,
        listener_registration: ListenerRegistration,
    );
}
