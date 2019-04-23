use std::os::raw::c_void;

use {ext_game_view, get_ui_bindings};

use ui;
use ui::{HasLocation, HasSize};

lazy_static! {
  static ref EXT_BINDING: ext_game_view = get_ui_bindings().game_view;
}

pub struct GameView(*mut c_void);

impl GameView {
  pub fn new(_self: *mut c_void) -> GameView {
    GameView(_self)
  }
}

impl HasSize for GameView {
  fn get_width(&self) -> i64 {
    (EXT_BINDING.get_width)(self.0)
  }

  fn get_height(&self) -> i64 {
    (EXT_BINDING.get_height)(self.0)
  }
}

impl HasLocation for GameView {
  fn get_left(&self) -> i64 {
    (EXT_BINDING.get_x)(self.0)
  }

  fn get_top(&self) -> i64 {
    (EXT_BINDING.get_y)(self.0)
  }
}

impl ui::GameView for GameView {}

impl Drop for GameView {
  fn drop(&mut self) {
    (EXT_BINDING.destroy)(self.0)
  }
}
