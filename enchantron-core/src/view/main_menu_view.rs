use crate::ui::Button;

use super::BaseView;

pub trait MainMenuView: 'static + Sized + Send + Sync + BaseView {
    type B: Button;

    fn get_start_new_game_button(&self) -> Self::B;

    fn transition_to_game_view(&self);
}
