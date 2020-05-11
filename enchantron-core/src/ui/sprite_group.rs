use super::{HasMutableVisibility, HasMutableZLevel, SpriteSource};

pub trait SpriteGroup:
    'static + HasMutableZLevel + HasMutableVisibility + SpriteSource + Unpin
{
    fn remove_from_parent(&self);
}
