use super::Texture;

pub trait Animation: 'static + Send + Sync + Unpin {
    type Texture: Texture;

    fn add_texture(&self, texture: &Self::Texture);

    fn set_is_loop(&self, is_loop: bool);

    fn set_name(&self, name: String);
}
