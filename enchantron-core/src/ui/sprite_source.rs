use super::Sprite;
use crate::native::Texture;

pub trait SpriteSource {
    type T: Texture;
    type S: Sprite<T = Self::T>;

    fn create_sprite(&self) -> Self::S;
}
