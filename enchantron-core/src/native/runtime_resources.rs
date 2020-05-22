use super::{Animations, Shaders, Textures};
use crate::view_types::ViewTypes;

pub struct RuntimeResources<T: ViewTypes> {
    textures: Textures<T>,
    animations: Animations<T>,
    shaders: Shaders<T>,
}

impl<T: ViewTypes> RuntimeResources<T> {
    pub fn new(
        textures: Textures<T>,
        animations: Animations<T>,
        shaders: Shaders<T>,
    ) -> RuntimeResources<T> {
        RuntimeResources {
            textures,
            animations,
            shaders,
        }
    }

    pub fn textures(&self) -> &Textures<T> {
        &self.textures
    }

    pub fn animations(&self) -> &Animations<T> {
        &self.animations
    }

    pub fn shaders(&self) -> &Shaders<T> {
        &self.shaders
    }
}
