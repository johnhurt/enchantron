use crate::ui::{
    HasDragHandlers, HasLayoutHandlers, HasMagnifyHandlers, HasViewport,
    SpriteSource,
};

use super::BaseView;

pub trait GameView:
    BaseView
    + SpriteSource
    + HasLayoutHandlers
    + HasDragHandlers
    + HasViewport
    + HasMagnifyHandlers
    + Sync
    + Send
    + 'static
{
}
