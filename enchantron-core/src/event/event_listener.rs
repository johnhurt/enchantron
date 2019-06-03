use super::{Event, EventKey};

pub trait EventListener<K: EventKey, E: Event<K>>:
    Sync + Send + 'static
{
    fn on_event(&self, event: &E);
}
