use super::{Texture, TextureLoader};
use crate::ui::Viewport;

pub trait SystemView: 'static + Sized + Sync + Send {
    type T: Texture;
    type TL: TextureLoader<T = Self::T>;
    type VP: Viewport;

    fn get_texture_loader(&self) -> Self::TL;

    fn get_viewport(&self) -> Self::VP;
}
