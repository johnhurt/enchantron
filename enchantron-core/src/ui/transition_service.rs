use crate::view::{LoadingView, MainMenuView, NativeView};

pub trait TransitionService: Send + Sync + 'static {
    type NV: NativeView;
    type LV: LoadingView;
    type MV: MainMenuView;

    fn transition_to_native_view(&self, view: &Self::NV, drop_current: bool);
    fn transition_to_loading_view(&self, view: &Self::LV, drop_current: bool);
    fn transition_to_main_menu_view(&self, view: &Self::MV, drop_current: bool);
}
