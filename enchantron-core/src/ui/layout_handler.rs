
macro_rules! create_layout_handler {
   (| $width:ident, $height:ident | $body:block) => {
    LayoutHandler::new(Box::new(move |$width, $height| $body ))
  };
}

pub struct LayoutHandler(Box<Fn(i64, i64) + 'static>);

impl LayoutHandler {
  pub fn new(_self: Box<Fn(i64, i64) + 'static>) -> LayoutHandler {
    LayoutHandler(_self)
  }

  pub fn on_layout(&self, width: i64, height: i64) {
    (self.0)(width, height)
  }
}


impl Drop for LayoutHandler {
  fn drop(&mut self) {
    println!("Dropping Layout Handler")
  }
}