use super::{ResourceLoader, Texture};
use crate::ui::TransitionService;
use crate::view::NativeView;

pub trait SystemInterop: 'static + Sync + Send {
    type T: Texture;
    type TL: ResourceLoader<T = Self::T>;
    type V: NativeView;
    type TS: TransitionService<V = Self::V>;

    fn get_resource_loader(&self) -> Self::TL;
    fn create_native_view(&self) -> Self::V;
    fn get_transition_service(&self) -> Self::TS;
}
