use std::sync::Arc;

use crate::native::RuntimeResources;
use crate::view_types::ViewTypes;

use super::{SpriteSource, SpriteSourceWrapper};

pub struct TerrainGenerator<T>
where
    T: ViewTypes,
{
    sprite_source: SpriteSourceWrapper<T>,
    runtime_resources: Arc<RuntimeResources<T::SystemView>>,
}

impl<T> TerrainGenerator<T>
where
    T: ViewTypes,
{
    pub fn new(
        sprite_source: SpriteSourceWrapper<T>,
        runtime_resources: Arc<RuntimeResources<T::SystemView>>,
    ) -> TerrainGenerator<T> {
        TerrainGenerator {
            sprite_source: sprite_source,
            runtime_resources: runtime_resources,
        }
    }

    pub fn on_viewport_change(&self) {
        self.sprite_source.create_sprite();
    }
}
