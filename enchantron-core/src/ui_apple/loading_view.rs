
use std::os::raw::c_void;

use ::{ext_loading_view, get_ui_bindings};

use ui;
use ui_apple::ProgressBar;

lazy_static! {
  static ref EXT_BINDING : ext_loading_view = get_ui_bindings().loading_view;
}

pub struct LoadingView(*mut c_void);

impl LoadingView {

  pub fn new(_self: *mut c_void) -> LoadingView {
    LoadingView(_self)
  }
}

impl ui::LoadingView for LoadingView {
  type P = ProgressBar;

  fn get_progress_indicator(&self) -> Self::P {
    ProgressBar::new((EXT_BINDING.get_progress_indicator)(self.0))
  }
}

impl Drop for LoadingView {

  fn drop(&mut self) {
    (EXT_BINDING.destroy)(self.0)
  }
}