use std::any::Any;
use std::fmt::Debug;

pub use self::event_bus::*;
pub use self::listener_registration::ListenerRegistration;

#[macro_use]
mod define_event_bus;

mod event_bus;
mod listener_registration;

pub trait Event: Any + Sync + Send + Debug + Clone + 'static {}
