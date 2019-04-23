use native::Texture;
use ui::Sprite;

pub trait SpriteSource {
  type T: Texture;
  type S: Sprite<T = Self::T>;

  fn create_sprite(&self) -> Self::S;
}