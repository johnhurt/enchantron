use super::SpriteSourceFn;
use crate::view_types::ViewTypes;

pub struct TerrainGenerator<T>
where
    T: ViewTypes,
{
    sprite_source: SpriteSourceFn<T::Sprite>,
}

impl<T> TerrainGenerator<T>
where
    T: ViewTypes,
{
    pub fn on_viewport_change(&self) {}
}
