use super::{Animations, Textures};
use crate::view_types::ViewTypes;

pub struct RuntimeResources<T: ViewTypes> {
    textures: Textures<T>,
    animations: Animations<T>,
}

impl<T: ViewTypes> RuntimeResources<T> {
    pub fn new(
        textures: Textures<T>,
        animations: Animations<T>,
    ) -> RuntimeResources<T> {
        RuntimeResources {
            textures,
            animations,
        }
    }

    pub fn textures(&self) -> &Textures<T> {
        &self.textures
    }

    pub fn animations(&self) -> &Animations<T> {
        &self.animations
    }
}
