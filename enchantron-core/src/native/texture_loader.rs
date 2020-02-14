use super::Texture;
use crate::util::ByteBuffer;

pub trait TextureLoader: Send + Sync + Unpin + 'static {
    type T: Texture;

    fn load_texture(&self, name: String) -> Self::T;

    fn load_texture_from_png_data(&self, png_data: ByteBuffer) -> Self::T;
}
