use super::{
    HasDragHandlers, HasLayoutHandlers, HasMagnifyHandlers, HasViewport,
    SpriteSource,
};

pub trait GameView:
    SpriteSource
    + HasLayoutHandlers
    + HasDragHandlers
    + HasViewport
    + HasMagnifyHandlers
    + Sync
    + Send
    + 'static
{
}
