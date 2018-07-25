use ui::Texture;

pub trait TextureLoader {
  type Tex : Texture;

  fn load_texture(&self, name: String) -> Self::Tex;
}