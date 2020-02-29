use super::{HasMutableZLevel, SpriteSource};

pub trait SpriteGroup: 'static + HasMutableZLevel + SpriteSource {
    fn remove_from_parent(&self);
}
