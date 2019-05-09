use std::any::Any;
use std::fmt::Debug;

pub use self::event_bus::EventBus;
pub use self::event_listener::EventListener;
pub use self::events::*;
pub use self::listener_registration::ListenerRegistration;
pub use self::event_key::EventKey;

mod event_key;
mod event_bus;
mod event_listener;
mod events;
mod listener_registration;

pub trait Event<K: EventKey> : Sync + Send + Debug + 'static {
    fn get_event_key(&self) -> K;
}
