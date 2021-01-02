use crate::view::{GameView, LoadingView, MainMenuView, NativeView};

pub trait TransitionService: Send + Sync + 'static {
    type NV: NativeView;
    type LV: LoadingView;
    type MV: MainMenuView;
    type GV: GameView;

    fn transition_to_native_view(&self, view: &Self::NV, drop_current: bool);
    fn transition_to_loading_view(&self, view: &Self::LV, drop_current: bool);
    fn transition_to_main_menu_view(&self, view: &Self::MV, drop_current: bool);
    fn transition_to_game_view(&self, view: &Self::GV, drop_current: bool);
}
