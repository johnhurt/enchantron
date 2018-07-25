use std::os::raw::c_void;

use ::{ext_handler_registration, get_ui_bindings};

use ui;

lazy_static! {
  static ref EXT_BINDING: ext_handler_registration 
      = get_ui_bindings().handler_registration;
}
pub struct HandlerRegistration(*mut c_void);

impl HandlerRegistration {
  pub fn new(_self: *mut c_void) -> HandlerRegistration {
    HandlerRegistration(_self)
  }
}

impl ui::HandlerRegistration for HandlerRegistration {
  fn deregister(&self) {
    (EXT_BINDING.deregister)(self.0)
  }
}

impl Drop for HandlerRegistration {
  fn drop(&mut self) {
    (EXT_BINDING.deregister)(self.0);
    (EXT_BINDING.destroy)(self.0);
  }
}
