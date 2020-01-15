use crate::game::{PerlinTerrain1, TerrainProvider, TerrainType};
use crate::model::IPoint;
use crate::native::RuntimeResources;
use crate::view_types::ViewTypes;

use std::sync::Arc;

pub struct TerrainTextureProvider<T: ViewTypes> {
    terrain_generator: Arc<PerlinTerrain1>,
    runtime_resources: Arc<RuntimeResources<T::SystemView>>,
}

impl<T> TerrainTextureProvider<T>
where
    T: ViewTypes,
{
    pub fn new(
        runtime_resources: Arc<RuntimeResources<T::SystemView>>,
    ) -> TerrainTextureProvider<T> {
        TerrainTextureProvider {
            runtime_resources: runtime_resources,
            terrain_generator: Arc::new(Default::default()),
        }
    }

    pub fn get_texture_at(&self, position: &IPoint) -> &T::Texture {
        match self.terrain_generator.get_for(position) {
            TerrainType::Grass => {
                self.runtime_resources.textures().overworld.grass()
            }
            TerrainType::Dirt => {
                self.runtime_resources.textures().overworld.dirt()
            }
        }
    }
}
