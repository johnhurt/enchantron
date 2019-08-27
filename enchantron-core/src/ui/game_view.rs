
use super::{HasDragHandlers, HasLayoutHandlers, SpriteSource, HasViewport};

pub trait GameView:
    SpriteSource + HasLayoutHandlers + HasDragHandlers + HasViewport + Sync + Send + 'static
{

}
