macro_rules! create_viewport_handler {
    (| $width:ident, $height:ident | $body:block) => {
        ViewportHandler::new(Box::new(move |$width, $height| $body))
    };
}

pub struct ViewportHandler(Box<Fn(i64, i64) + 'static>);

impl ViewportHandler {
    pub fn new(_self: Box<Fn(i64, i64) + 'static>) -> ViewportHandler {
        ViewportHandler(_self)
    }

    pub fn on_layout(&self, width: i64, height: i64) {
        (self.0)(width, height)
    }
}

impl Drop for ViewportHandler {
    fn drop(&mut self) {
        println!("Dropping Viewport Handler")
    }
}
