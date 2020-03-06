use super::{HandlerRegistration, MultiDragHandler};

pub trait HasMultiDragHandlers: 'static {
    type R: HandlerRegistration;

    fn add_multi_drag_handler(&self, handler: MultiDragHandler) -> Self::R;
}
