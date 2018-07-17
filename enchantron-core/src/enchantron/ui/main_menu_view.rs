use enchantron::ui;

pub trait MainMenuView : 'static {
  fn get_start_game_button(&self) -> ui::Button;
}