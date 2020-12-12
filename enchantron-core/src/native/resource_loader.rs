use super::{Animation, Texture};
use crate::util::ByteBuffer;

pub trait ResourceLoader: Send + Sync + Unpin + 'static {
    type T: Texture;
    type A: Animation;

    fn load_texture(&self, name: String) -> Self::T;

    fn load_texture_from_png_data(&self, png_data: ByteBuffer) -> Self::T;

    fn create_animation(&self) -> Self::A;
}
