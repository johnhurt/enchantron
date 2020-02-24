use crate::game::{PerlinTerrain1, TerrainProvider, TerrainType};
use crate::img::PngGenerator;
use crate::model::{IPoint, IRect, ISize};
use crate::native::{RuntimeResources, Texture, TextureLoader};
use crate::util::{ByteBuffer, ValueRect};
use crate::view_types::ViewTypes;

use std::ptr::copy_nonoverlapping;
use std::sync::Arc;

const BROWN_BYTES: [u8; 3] = [0x65, 0x43, 0x21];
const GREEN_BYTES: [u8; 3] = [0x90, 0x33, 0x90];

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
    unsafe fn fill_rect_with_terrain(
        &self,
        target: *mut u8,
        image_width: usize,
        tile_width: usize,
        tile_height: usize,
        terrain: &TerrainType,
    ) {
        let src_bytes = match terrain {
            TerrainType::Grass => GREEN_BYTES.as_ptr(),
            TerrainType::Dirt => BROWN_BYTES.as_ptr(),
        };

        for row in 0..tile_height {
            for col in 0..tile_width {
                copy_nonoverlapping(src_bytes, target, 3);
                target.add(3);
            }
            target.add(3 * (image_width - tile_width));
        }
    }

    pub fn get_texture_data_for_rect(
        &self,
        rect: &IRect,
        texture_size: &ISize,
    ) -> ByteBuffer {
        let x_tile_pixels = rect.size.width / texture_size.width;
        let y_tile_pixels = rect.size.height / texture_size.height;

        debug_assert_eq!(x_tile_pixels * texture_size.width, rect.size.width);
        debug_assert_eq!(y_tile_pixels * texture_size.height, rect.size.height);

        let mut data = Vec::<u8>::with_capacity(texture_size.area() * 3);

        unsafe {
            data.set_len(data.capacity());
            let data_ptr = data.as_mut_ptr();

            self.terrain_generator
                .get_for_rect(rect)
                .for_each_value_coord(|coord, terrain| {
                    let curr_ptr = data_ptr.add(
                        3 * x_tile_pixels
                            * (coord.x
                                + coord.y * y_tile_pixels * rect.size.width),
                    );
                    self.fill_rect_with_terrain(
                        curr_ptr,
                        texture_size.width,
                        x_tile_pixels,
                        y_tile_pixels,
                        terrain,
                    )
                });
        }

        let mut result = ByteBuffer::new(Vec::<u8>::with_capacity(4096));

        PngGenerator::get_png(data.as_slice(), texture_size, &mut result);

        //self.texture_loader.load_texture_from_png_data(result)
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn test_generate_texture_data() {
        // let this =

        // let result =

        // let name =
        //     format!("/Users/kguthrie/Downloads/img_{}.png", result.len());

        // let mut pos = 0;
        // let mut buffer = File::create(&name).expect("");

        // while pos < result.len() {
        //     let bytes_written = buffer.write(&result[pos..]).expect("");
        //     pos += bytes_written;
        // }
    }
}
