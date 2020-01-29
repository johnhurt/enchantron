use crate::util::ByteBuffer;
use png::{BitDepth, ColorType, Compression, Encoder};
use rand::{thread_rng, Rng};

pub struct PngGenerator {}

impl PngGenerator {
    pub fn get_png() -> ByteBuffer {
        let mut data = Vec::<u8>::new();
        {
            let mut encoder = Encoder::new(&mut data, 64, 64);
            encoder.set_color(ColorType::RGB);
            encoder.set_depth(BitDepth::Four);
            encoder.set_compression(Compression::Default);

            let mut writer = encoder.write_header().unwrap_or_else(|e| {
                error!("Failed to write png header data: {:?}", e);
                panic!("Failed to write png header");
            });

            let mut rng = thread_rng();

            let mut image_data = Vec::<u8>::with_capacity(64 * 64 * 4 * 3 / 8);
            for _ in 0..image_data.capacity() {
                image_data.push(rng.gen())
            }

            writer.write_image_data(image_data.as_slice()).unwrap();
        }

        ByteBuffer::new(data)
    }
}

#[test]
fn test_png() {
    PngGenerator::get_png();
}
