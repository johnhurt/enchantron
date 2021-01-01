use super::{ClickHandler, HasClickHandlers};
use crate::model::Rect;
use crate::view_types::ViewTypes;
use crate::widget;

pub trait Button: HasClickHandlers + Send + Sync + 'static {}

widget!(Button<T> {
    sprites {
        outline
    }

    private {
        click_handlers: Vec<ClickHandler>,
        rect: Rect
    }
});


impl <T> Button for ButtonPublic<T> where T: ViewTypes {}

impl <T> HasClickHandlers for ButtonPublic<T> where T: ViewTypes {

    type R =

    fn add_click_handler(&self, handler: ClickHandler) -> Self::R {

    }
}