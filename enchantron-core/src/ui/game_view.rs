use super::{HasDragHandlers, HasLayoutHandlers, HasViewport, SpriteSource};

pub trait GameView:
    SpriteSource
    + HasLayoutHandlers
    + HasDragHandlers
    + HasViewport
    + Sync
    + Send
    + 'static
{
}
