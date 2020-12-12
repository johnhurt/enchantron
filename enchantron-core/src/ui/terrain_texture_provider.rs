use crate::application_context::Ao;
use crate::game::constants;
use crate::game::{PerlinTerrain1, TerrainProvider, TerrainType};
use crate::img::PngGenerator;
use crate::model::{IRect, ISize};
use crate::native::{ResourceLoader, RuntimeResources};
use crate::util::ByteBuffer;
use crate::view_types::ViewTypes;

const BROWN_BYTES: [u8; 3] = constants::DIRT_BROWN_RGB;
const GREEN_BYTES: [u8; 3] = constants::GRASS_GREEN_RGB;

lazy_static! {
    static ref PALETTE: Vec<u8> = BROWN_BYTES
        .iter()
        .cloned()
        .chain(GREEN_BYTES.iter().cloned())
        .collect();
}

fn get_texture_data_for_rect(
    rect: &IRect,
    texture_size: &ISize,
    terrain_generator: &impl TerrainProvider,
) -> ByteBuffer {
    let texture_size = &rect.size;

    let x_tile_pixels = texture_size.width / rect.size.width;
    let y_tile_pixels = texture_size.height / rect.size.height;

    debug_assert_eq!(x_tile_pixels * rect.size.width, texture_size.width);
    debug_assert_eq!(y_tile_pixels * rect.size.height, texture_size.height);

    let mut data = Vec::<u8>::with_capacity(texture_size.area());

    unsafe {
        data.set_len(data.capacity());
        let mut data_ptr = data.as_mut_ptr();

        let terrain_data = terrain_generator.get_for_rect(rect);

        terrain_data.for_each_value_coord(|coord, (val, terrain)| {
            let terrain_val = match terrain {
                TerrainType::Dirt => 0u8,
                TerrainType::Grass => 1,
            };
            *data_ptr = terrain_val;
            data_ptr = data_ptr.add(1);
        });
    }

    let mut result = ByteBuffer::new(Vec::<u8>::with_capacity(4096));

    PngGenerator::get_png_indexed(
        data.as_slice(),
        PALETTE.clone(),
        texture_size,
        &mut result,
    );

    result
}
pub struct TerrainTextureProvider<T: ViewTypes> {
    terrain_generator: PerlinTerrain1,
    runtime_resources: Ao<RuntimeResources<T>>,
    texture_loader: T::ResourceLoader,
}

impl<T> TerrainTextureProvider<T>
where
    T: ViewTypes,
{
    pub fn new(
        runtime_resources: Ao<RuntimeResources<T>>,
        texture_loader: T::ResourceLoader,
    ) -> TerrainTextureProvider<T> {
        TerrainTextureProvider {
            runtime_resources,
            texture_loader,
            terrain_generator: Default::default(),
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
                &self.terrain_generator,
            ),
        )
    }
}

#[allow(dead_code, unused_imports)]
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
        let image_size = ISize::new(16, 16);

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
