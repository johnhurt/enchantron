use std::any::Any;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;

pub type Selector<W> =
    Box<dyn Fn(&mut dyn Any) -> &mut W + Send + Sync + 'static>;
pub type AnyConsumer = Box<dyn FnOnce(&mut dyn Any) + Send + Sync + 'static>;

#[derive(derive_new::new)]
pub struct DynActionSink<W> {
    pub(crate) sender: Sender<AnyConsumer>,
    pub(crate) selector: Selector<W>,
}

impl<W> DynActionSink<W>
where
    W: Send + Sync + 'static,
{
    pub fn send(
        self: &Arc<Self>,
        action: impl FnOnce(&mut W) + Send + Sync + 'static,
    ) {
        let copy = self.clone();
        let _ = self
            .sender
            .try_send(Box::new(move |any| action((copy.selector)(any))));
    }
}
