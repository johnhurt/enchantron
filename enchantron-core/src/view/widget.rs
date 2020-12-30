use std::any::Any;
use std::sync::Arc;

pub type WidgetSelector<W> =
    Arc<dyn Fn(&mut dyn Any) -> &mut W + Send + Sync + 'static>;

pub type AnyConsumer = Box<dyn FnOnce(&mut dyn Any) + Send + Sync + 'static>;
