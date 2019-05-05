use super::{HasDragHandlers, HasLayoutHandlers, SpriteSource};

pub trait GameView:
    SpriteSource + HasLayoutHandlers + HasDragHandlers + 'static
{
}
