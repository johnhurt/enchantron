use crate::game::constants;
use crate::game::{PerlinTerrain1, TerrainProvider, TerrainType};
use crate::img::PngGenerator;
use crate::model::{IPoint, IRect, ISize};
use crate::native::{ResourceLoader, RuntimeResources};
use crate::util::ByteBuffer;
use crate::view_types::ViewTypes;
use std::ptr::copy_nonoverlapping;
use std::sync::Arc;

const BROWN_BYTES: [u8; 3] = constants::DIRT_BROWN_RGB;
const GREEN_BYTES: [u8; 3] = constants::GRASS_GREEN_RGB;

/// Write to the given image-data slice in the locations corresponding to
/// the
unsafe fn fill_rect_with_terrain(
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

    let mut target = target;

    for _ in 0..tile_height {
        for _ in 0..tile_width {
            copy_nonoverlapping(src_bytes, target, 3);
            target = target.add(3);
        }
        target = target.add(3 * (image_width - tile_width));
    }
}

fn get_texture_data_for_rect(
    rect: &IRect,
    texture_size: &ISize,
    terrain_generator: &impl TerrainProvider,
) -> ByteBuffer {
    let ref texture_size = rect.size;

    let x_tile_pixels = texture_size.width / rect.size.width;
    let y_tile_pixels = texture_size.height / rect.size.height;

    debug_assert_eq!(x_tile_pixels * rect.size.width, texture_size.width);
    debug_assert_eq!(y_tile_pixels * rect.size.height, texture_size.height);

    let mut data = Vec::<u8>::with_capacity(texture_size.area() * 3);

    unsafe {
        data.set_len(data.capacity());
        let data_ptr = data.as_mut_ptr();

        let terrain_data = terrain_generator.get_for_rect(rect);

        terrain_data.for_each_value_coord(|coord, (val, terrain)| {
            let curr_ptr = data_ptr.add(
                3 * x_tile_pixels
                    * (coord.x + coord.y * y_tile_pixels * rect.size.width),
            );
            fill_rect_with_terrain(
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

    result
}
pub struct TerrainTextureProvider<T: ViewTypes> {
    terrain_generator: Arc<PerlinTerrain1>,
    runtime_resources: Arc<RuntimeResources<T>>,
    texture_loader: T::ResourceLoader,
}

impl<T> TerrainTextureProvider<T>
where
    T: ViewTypes,
{
    pub fn new(
        runtime_resources: Arc<RuntimeResources<T>>,
        texture_loader: T::ResourceLoader,
    ) -> TerrainTextureProvider<T> {
        TerrainTextureProvider {
            runtime_resources: runtime_resources,
            texture_loader: texture_loader,
            terrain_generator: Arc::new(Default::default()),
        }
    }

    pub fn get_texture_for_rect(
        &self,
        rect: &IRect,
        texture_size: &ISize,
    ) -> T::Texture {
        self.texture_loader.load_texture_from_png_data(
            get_texture_data_for_rect(
                rect,
                texture_size,
                &*self.terrain_generator,
            ),
        )
    }

    /// Get a reference to the terrain shader
    pub fn get_terrain_shader(&self) -> &T::Shader {
        &self.runtime_resources.shaders().terrain_shader
    }
}

#[allow(dead_code)]
#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;
    use std::time::SystemTime;

    #[test]
    fn test_generate_texture_data() {
        let terrain_generator = PerlinTerrain1::default();

        let terrain_rect = IRect::new(0, 0, 16, 16);
        let image_size = ISize::new(128, 128);

        let now = SystemTime::now();

        let _ = get_texture_data_for_rect(
            &terrain_rect,
            &image_size,
            &terrain_generator,
        );

        println!("Terrain texture generation time: {:?}", now.elapsed());

        //panic!("to get stdout");

        // let name = format!("/Users/kguthrie/Downloads/img.png");

        // let mut pos = 0;
        // let mut buffer = File::create(&name).expect("");

        // while pos < result.len() {
        //     let bytes_written = buffer.write(&result[pos..]).expect("");
        //     pos += bytes_written;
        // }
    }
}
