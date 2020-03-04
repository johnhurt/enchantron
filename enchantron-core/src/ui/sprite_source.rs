use super::{Sprite, SpriteGroup};
use crate::native::Texture;

pub trait SpriteSource: Send + Sync {
    type T: Texture;
    type S: Sprite<T = Self::T>;
    type G: SpriteGroup<T = Self::T, S = Self::S>;

    fn create_sprite(&self) -> Self::S;

    fn create_group(&self) -> Self::G;
}
