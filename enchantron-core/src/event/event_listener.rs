use super::{Event, EventKey, HasListenerRegistrations};

pub trait EventListener<K: EventKey, E: Event<K>>:
    HasListenerRegistrations + Sync + Send + 'static
{
    fn on_event(&self, event: &E);
}
