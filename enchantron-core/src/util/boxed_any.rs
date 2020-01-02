use std::any::Any;

pub type BoxedAny = Box<dyn Any + 'static + Send + Sync>;
