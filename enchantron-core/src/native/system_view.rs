use super::{ResourceLoader, Texture};

pub trait SystemView: 'static + Sync + Send {
    type T: Texture;
    type TL: ResourceLoader<T = Self::T>;

    fn get_resource_loader(&self) -> Self::TL;
}
