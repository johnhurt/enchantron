use super::{HasClickHandlers, ClickHandler};
use crate::view_types::ViewTypes;
use crate::widget;
use crate::model::Rect;


pub trait Button : HasClickHandlers + Send + Sync + 'static {}

widget!(Button<T> {
    outline: T::Sprite
    click_handlers: Vec<ClickHandler>,
    rect: Rect;
});