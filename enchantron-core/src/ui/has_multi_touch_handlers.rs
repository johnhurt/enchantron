use super::{HandlerRegistration, MultiTouchHandler};

pub trait HasMultiTouchHandlers: 'static {
    type R: HandlerRegistration;

    fn add_multi_touch_handler(&self, handler: MultiTouchHandler) -> Self::R;
}
