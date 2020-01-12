use std::any::Any;

pub type BoxedAny = Box<UnboxedAny>;
pub type UnboxedAny = dyn Any + Send + Sync;
pub trait AnyClone: Any + Clone {}
pub type UnboxedAnyClone = dyn AnyClone + Send + Sync;
pub type BoxedAnyClone = Box<UnboxedAnyClone>;

impl<T> AnyClone for T where T: Any + Clone {}
