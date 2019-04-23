

use native::{
    Texture,
    TextureLoader
};

pub trait SystemView : 'static + Sized {
  type T : Texture;
  type TL : TextureLoader<T = Self::T>;


  fn get_texture_loader(&self) -> Self::TL;
}
