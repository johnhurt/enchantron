use crate::ui::{
    HasLayoutHandlers, HasMagnifyHandlers, HasMultiDragHandlers, HasViewport,
    SpriteSource,
};

use super::BaseView;

pub trait GameView:
    BaseView
    + SpriteSource
    + HasLayoutHandlers
    + HasMultiDragHandlers
    + HasViewport
    + HasMagnifyHandlers
    + Sync
    + Send
    + 'static
{
}
