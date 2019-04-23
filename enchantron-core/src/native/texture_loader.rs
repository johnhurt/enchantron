use native::Texture;

pub trait TextureLoader: 'static {
  type T : Texture;

  fn load_texture(&self, name: String) -> Self::T;
}