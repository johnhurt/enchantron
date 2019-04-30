use native::{SystemView, Textures};

pub struct RuntimeResources<S: SystemView> {
    textures: Textures<S::T>,
}

impl<S: SystemView> RuntimeResources<S> {
    pub fn new(textures: Textures<S::T>) -> RuntimeResources<S> {
        RuntimeResources { textures: textures }
    }

    pub fn textures(&self) -> &Textures<S::T> {
        &self.textures
    }
}
