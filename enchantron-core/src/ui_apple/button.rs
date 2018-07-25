
use std::os::raw::c_void;

use ui;
use ui::{HasClickHandlers, HasText};

use ::{ext_button, get_ui_bindings};

use ui_apple::{HandlerRegistration, ClickHandler, RustString};

lazy_static! {
  static ref EXT_BINDING : ext_button = get_ui_bindings().button;
}

pub struct Button(*mut c_void);

impl Button {
    pub fn new (_self: *mut c_void) -> Button {
        Button(_self)
    }
}

impl ui::Button for Button {}

impl HasClickHandlers for Button {
  type R = HandlerRegistration;

  fn add_click_handler(&self, handler: Box<Fn() + 'static>) -> Self::R {

    let click_handler = ClickHandler::new(handler);

    HandlerRegistration::new((EXT_BINDING.add_click_handler)(
        self.0, 
        Box::into_raw(Box::new(click_handler))))
  }
}

impl HasText for Button {

  fn get_text(&self) -> String {
    let text_ptr = (EXT_BINDING.get_text)(self.0);
            
    let text = unsafe { Box::from_raw(text_ptr) };

    text.to_string()
  }

  fn set_text(&self, text: String) {
    let rust_text = RustString::new(text);
    (EXT_BINDING.set_text)(self.0, Box::into_raw(Box::new(rust_text)));
  }
}

impl Drop for Button {
  fn drop(&mut self) {
    (EXT_BINDING.destroy)(self.0)
  }
}