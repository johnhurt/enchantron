
use std::os::raw::c_void;

use ui;

use ui_impl::Button;

use ::{ext_main_menu_view, get_ui_bindings};

lazy_static! {
  static ref EXT_BINDING : ext_main_menu_view = get_ui_bindings().main_menu_view;
}

pub struct MainMenuView(*mut c_void);

impl MainMenuView {
  pub fn new(_self: *mut c_void) -> MainMenuView {
    MainMenuView(_self)
  }
}

impl ui::MainMenuView for MainMenuView {
  type B = Button;
  
  fn get_start_game_button(&self) -> Self::B {
    Button::new((EXT_BINDING.get_start_game_button)(self.0))
  }
}


impl Drop for MainMenuView {
  fn drop(&mut self) {
    (EXT_BINDING.destroy)(self.0)
  }
}
