macro_rules! create_layout_handler {
    (| $width:ident, $height:ident | $body:block) => {
        LayoutHandler::new(Box::new(move |$width, $height| $body))
    };
}

pub struct LayoutHandler(Box<dyn Fn(i64, i64) + 'static + Send>);

impl LayoutHandler {
    pub fn new(_self: Box<dyn Fn(i64, i64) + 'static + Send>) -> LayoutHandler {
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
