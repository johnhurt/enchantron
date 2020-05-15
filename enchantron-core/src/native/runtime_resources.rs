use super::Textures;
use crate::view_types::ViewTypes;

pub struct RuntimeResources<T: ViewTypes> {
    textures: Textures<T::Texture>,
}

impl<T: ViewTypes> RuntimeResources<T> {
    pub fn new(textures: Textures<T::Texture>) -> RuntimeResources<T> {
        RuntimeResources { textures: textures }
    }

    pub fn textures(&self) -> &Textures<T::Texture> {
        &self.textures
    }
}
