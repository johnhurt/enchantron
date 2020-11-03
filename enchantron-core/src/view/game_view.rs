use crate::ui::{
    HasLayoutHandlers, HasMagnifyHandlers, HasMultiTouchHandlers, HasViewport,
    SpriteSource,
};

use super::BaseView;

pub trait GameView:
    BaseView
    + SpriteSource
    + HasLayoutHandlers
    + HasMultiTouchHandlers
    + HasViewport
    + HasMagnifyHandlers
    + Sync
    + Send
    + 'static
{
}
