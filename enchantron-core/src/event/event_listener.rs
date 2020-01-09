use super::{Event, EventKey};

#[async_trait]
pub trait EventListener<K: EventKey, E: Event<K>>:
    Sync + Send + 'static
{
    async fn on_event(&self, event: &E);
}
