use ui::Button;

pub trait MainMenuView: 'static + Sized {
    type B: Button;

    fn get_start_new_game_button(&self) -> Self::B;

    fn transition_to_game_view(&self);
}
