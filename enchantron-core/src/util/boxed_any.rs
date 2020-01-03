use std::any::Any;

pub type BoxedAny = Box<UnboxedAny>;
pub type UnboxedAny = dyn Any + 'static + Send + Sync;