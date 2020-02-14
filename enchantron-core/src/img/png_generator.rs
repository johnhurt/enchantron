use crate::util::ByteBuffer;
use crate::model::ISize;
use png::{BitDepth, ColorType, Compression, Encoder};

pub struct PngGenerator {}

impl PngGenerator {
    pub fn get_png(data: &[u8], size: &ISize, target: &mut Vec<u8>) {
        let mut encoder = Encoder::new(target, size.width as u32, size.height as u32);
        encoder.set_color(ColorType::RGB);
        encoder.set_depth(BitDepth::Eight);
        encoder.set_compression(Compression::Fast);

        let mut writer = encoder.write_header().unwrap_or_else(|e| {
            error!("Failed to write png header data: {:?}", e);
            panic!("Failed to write png header");
        });

        writer.write_image_data(data).unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::{thread_rng, RngCore};
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn test_png() {
        let mut result = ByteBuffer::new(Vec::<u8>::with_capacity(4096*16));
        let mut data = Vec::<u8>::with_capacity(128 * 128 * 3);

        unsafe {
            data.set_len(128 * 128 * 3);
        }

        let mut r = thread_rng();
        r.fill_bytes(data.as_mut_slice());

        PngGenerator::get_png(data.as_slice(), 128, 128, &mut result);

        let name =
            format!("/Users/kguthrie/Downloads/img_{}.png", result.len());

        let mut pos = 0;
        let mut buffer = File::create(&name).expect("");

        while pos < result.len() {
            let bytes_written = buffer.write(&result[pos..]).expect("");
            pos += bytes_written;
        }
    }
}
