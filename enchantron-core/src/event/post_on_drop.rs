use super::{Event, EventBus};

#[derive(derive_new::new)]
pub struct PostOnDrop<E: Event> {
    event: E,
    event_bus: EventBus,
}

impl<E> Drop for PostOnDrop<E>
where
    E: Event,
{
    fn drop(&mut self) {
        self.event_bus.post(self.event)
    }
}
