use crate::model::ISize;
use png::{BitDepth, ColorType, Encoder};

pub struct PngGenerator {}

impl PngGenerator {
    pub fn get_png_indexed(
        indexed_data: &[u8],
        palette_data: Vec<u8>,
        size: &ISize,
        target: &mut Vec<u8>,
    ) {
        let mut encoder =
            Encoder::new(target, size.width as u32, size.height as u32);
        encoder.set_color(ColorType::Indexed);
        encoder.set_depth(BitDepth::Eight);
        //encoder.set_compression(png::Compression::Fast);
        encoder.set_palette(palette_data);

        let mut writer = encoder.write_header().unwrap_or_else(|e| {
            error!("Failed to write png header data: {:?}", e);
            panic!("Failed to write png header");
        });

        writer.write_image_data(indexed_data).unwrap();
    }

    pub fn get_png(data: &[u8], size: &ISize, target: &mut Vec<u8>) {
        let mut encoder =
            Encoder::new(target, size.width as u32, size.height as u32);
        encoder.set_color(ColorType::RGB);
        encoder.set_depth(BitDepth::Eight);
        encoder.set_compression(png::Compression::Fast);

        let mut writer = encoder.write_header().unwrap_or_else(|e| {
            error!("Failed to write png header data: {:?}", e);
            panic!("Failed to write png header");
        });

        writer.write_image_data(data).unwrap();
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod test {
    use super::*;
    use crate::util::ByteBuffer;
    use rand::{thread_rng, RngCore};
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn test_png() {
        let mut result = ByteBuffer::new(Vec::<u8>::with_capacity(4096 * 16));
        let size = ISize::new(128, 128);
        let mut data = Vec::<u8>::with_capacity(size.area() * 3);

        unsafe {
            data.set_len(128 * 128 * 3);
        }

        let mut r = thread_rng();
        r.fill_bytes(data.as_mut_slice());

        PngGenerator::get_png(data.as_slice(), &size, &mut result);

        let name = format!("target/img_{}.png", result.len());

        let mut pos = 0;
        let mut buffer = File::create(&name).expect("");

        while pos < result.len() {
            let bytes_written = buffer.write(&result[pos..]).expect("");
            pos += bytes_written;
        }
    }

    #[test]
    fn test_indexed_png() {
        let mut result = ByteBuffer::new(Vec::<u8>::with_capacity(4096 * 16));
        let size = ISize::new(128, 128);
        let mut data = Vec::<u8>::with_capacity(size.area());
        let palette = vec![0x34u8, 0xdd, 0x9a, 0xff, 0, 0];

        unsafe {
            data.set_len(128 * 128);
        }

        for col in 0..128 {
            for row in 0..128 {
                data[row * 128 + col] = if col % 2 == row % 2 { 1 } else { 0 };
            }
        }

        PngGenerator::get_png_indexed(
            data.as_slice(),
            palette,
            &size,
            &mut result,
        );

        let name = format!("target/img_{}.png", result.len());

        let mut pos = 0;
        let mut buffer = File::create(&name).expect("");

        while pos < result.len() {
            let bytes_written = buffer.write(&result[pos..]).expect("");
            pos += bytes_written;
        }
    }
}
