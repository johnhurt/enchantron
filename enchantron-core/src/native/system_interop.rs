use super::{ResourceLoader, Texture};
use crate::view::{LoadingView, MainMenuView, NativeView};
use crate::{ui::TransitionService, view::GameView};

pub trait SystemInterop: 'static + Sync + Send {
    type T: Texture;
    type TL: ResourceLoader<T = Self::T>;
    type TS: TransitionService<NV = Self::NV, LV = Self::LV>;
    type NV: NativeView;
    type LV: LoadingView;
    type MV: MainMenuView;
    type GV: GameView;

    fn get_resource_loader(&self) -> Self::TL;
    fn get_transition_service(&self) -> Self::TS;
    fn create_native_view(&self) -> Self::NV;
    fn create_loading_view(&self) -> Self::LV;
    fn create_main_menu_view(&self) -> Self::MV;
    fn create_game_view(&self) -> Self::GV;
}
