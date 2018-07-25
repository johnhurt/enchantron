
use ui::Button;

pub trait MainMenuView : 'static + Sized {
  type B : Button;

  fn get_start_game_button(&self) -> Self::B;

}