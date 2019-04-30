use ui::{HandlerRegistration, LayoutHandler};

pub trait HasLayoutHandlers: 'static {
    type R: HandlerRegistration;

    fn add_layout_handler(&self, handler: LayoutHandler) -> Self::R;
}
