use super::{ResourceLoader, Texture};
use crate::ui::TransitionService;
use crate::view::{LoadingView, NativeView};

pub trait SystemInterop: 'static + Sync + Send {
    type T: Texture;
    type TL: ResourceLoader<T = Self::T>;
    type TS: TransitionService<NV = Self::NV, LV = Self::LV>;
    type NV: NativeView;
    type LV: LoadingView;

    fn get_resource_loader(&self) -> Self::TL;
    fn get_transition_service(&self) -> Self::TS;
    fn create_native_view(&self) -> Self::NV;
    fn create_loading_view(&self) -> Self::LV;
}
