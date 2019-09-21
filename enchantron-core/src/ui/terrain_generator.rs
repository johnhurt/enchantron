use std::sync::Arc;

use super::SpriteSource;
use crate::view_types::ViewTypes;

pub struct TerrainGenerator<T>
where
    T: ViewTypes,
{
    sprite_source: Arc<dyn SpriteSource<S = T::Sprite, T = T::Texture>>
}

impl<T> TerrainGenerator<T>
where
    T: ViewTypes,
{
    pub fn new(sprite_source: Arc<dyn SpriteSource<S = T::Sprite, T = T::Texture>>) -> TerrainGenerator<T> {
        TerrainGenerator {
            sprite_source: sprite_source
        }
    }

    pub fn on_viewport_change(&self) {
        (self.sprite_source)();
    }
}
