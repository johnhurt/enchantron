
pub struct ClickHandler(Box<Fn() + 'static>);

impl ClickHandler {
  pub fn new(_self: Box<Fn() + 'static>) -> ClickHandler {
    ClickHandler(_self)
  }

  pub fn on_click(&self) {
    (self.0)()
  }
}

pub extern "C" fn handle_click(handler_ptr: *mut ClickHandler) {
  let handler = unsafe { &*handler_ptr };
  handler.on_click()
}

pub extern "C" fn drop_click_handler(handler_ptr: *mut ClickHandler) {
  let _ = unsafe { Box::from_raw(handler_ptr) };
}