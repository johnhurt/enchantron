
use ::ext_click_handler;

pub static EXT_BINDING : ext_click_handler = ext_click_handler {
  on_click: handle_click,
  drop: drop_click_handler
};

pub struct ClickHandler(Box<Fn() + 'static>);

impl ClickHandler {
  pub fn new(_self: Box<Fn() + 'static>) -> ClickHandler {
    ClickHandler(_self)
  }

  pub fn on_click(&self) {
    (self.0)()
  }
}


impl Drop for ClickHandler {
  fn drop(&mut self) {
    println!("Dropping Click Handler")
  }
}

extern "C" fn handle_click(handler_ptr: *mut ClickHandler) {
  let handler = unsafe { &*handler_ptr };
  handler.on_click()
}

extern "C" fn drop_click_handler(handler_ptr: *mut ClickHandler) {
  let _ = unsafe { Box::from_raw(handler_ptr) };
}

