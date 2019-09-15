use super::{HandlerRegistration, MagnifyHandler};

pub trait HasMagnifyHandlers: 'static {
    type R: HandlerRegistration;

    fn add_magnify_handler(&self, handler: MagnifyHandler) -> Self::R;
}
