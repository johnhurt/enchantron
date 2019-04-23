use std::os::raw::c_void;

use ::{ext_progress_bar, get_ui_bindings};

use ui;
use ui::{HasText, HasIntValue};

use ui_apple::{HandlerRegistration, ClickHandler, RustString, SwiftString};

lazy_static! {
  static ref EXT_BINDING : ext_progress_bar = get_ui_bindings().progress_bar;
}

pub struct ProgressBar(*mut c_void);

impl ProgressBar {
  pub fn new(_self: *mut c_void) -> ProgressBar {
    ProgressBar(_self)
  }
}

impl ui::ProgressBar for ProgressBar {}

impl HasIntValue for ProgressBar {
  fn get_int_value(&self) -> i64 {
    (EXT_BINDING.get_int_value)(self.0)
  }

  fn set_int_value(&self, value: i64) {
    (EXT_BINDING.set_int_value)(self.0, value)
  }
}

impl HasText for ProgressBar {

  fn get_text(&self) -> String {
    let text = SwiftString::new((EXT_BINDING.get_text)(self.0));

    text.to_string()
  }

  fn set_text(&self, text: String) {
    let rust_text = RustString::new(text);
    (EXT_BINDING.set_text)(self.0, Box::into_raw(Box::new(rust_text)));
  }
}

impl Drop for ProgressBar {
  fn drop(&mut self) {
    (EXT_BINDING.destroy)(self.0)
  }
}