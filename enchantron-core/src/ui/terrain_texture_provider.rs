use crate::game::{PerlinTerrain1, TerrainProvider, TerrainType};
use crate::model::{IPoint, IRect, ISize};
use crate::native::{RuntimeResources, Texture, TextureLoader};
use crate::util::ValueRect;
use crate::view_types::ViewTypes;
use crate::img::PngGenerator;

use std::sync::Arc;

pub struct TerrainTextureProvider<T: ViewTypes> {
    terrain_generator: Arc<PerlinTerrain1>,
    runtime_resources: Arc<RuntimeResources<T::SystemView>>,
    texture_loader: T::TextureLoader,
}

impl<T> TerrainTextureProvider<T>
where
    T: ViewTypes,
{
    pub fn new(
        runtime_resources: Arc<RuntimeResources<T::SystemView>>,
        texture_loader: T::TextureLoader,
    ) -> TerrainTextureProvider<T> {
        TerrainTextureProvider {
            runtime_resources: runtime_resources,
            texture_loader: texture_loader,
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

    /// Write to the given image-data slice in the locations corresponsing to
    /// the
    fn fill_rect_with_terraint(&self,
        target: &mut [u8],
        point: &Upoint,
        image_size: &ISize,
        terrain_size: &ISize,
        terrain: &Terrain) {

    }

    pub fn get_textures_for_rect(
        &self,
        rect: &IRect,
        texture_size: &ISize,
    ) -> T::Texture {

        let x_tile_pixels = rect.size.width / texture_size.width;
        let y_tile_pixels = rect.size.height / testure_size.height;

        debug_assert_eq!(x_tile_pixels * texture_size.width = rect.size.width);
        debug_assert_eq!(y_tile_pixels * texture_size.height = rect.size.height);

        let mut data = Vec::<u8>::with_capacity(texture_size.area());

        self.terrain_generator.get_for_rect(rect)
            .for_each_value_coord(|coord, terrain| {
                match terrain {
                    TerrainType::Grass => {
                        self.runtime_resources.textures().overworld.grass()
                    }
                    TerrainType::Dirt => {
                        self.runtime_resources.textures().overworld.dirt()
                    }
                }
            });

        let mut result = ByteBuffer::new(Vec::<u8>::with_capacity(4096));

        PngGenerator::get_png(texture_size, &mut result);

        self.texture_loader.load_texture_from_png_data(result)
    }
}
