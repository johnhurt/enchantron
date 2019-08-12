use super::{HasDragHandlers, HasLayoutHandlers, HasViewport, SpriteSource};

pub trait GameView:
    SpriteSource + HasLayoutHandlers + HasDragHandlers + Sync + Send + 'static
{
}
