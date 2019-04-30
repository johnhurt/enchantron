macro_rules! create_click_handler {
    ($body:block) => {
        ClickHandler::new(Box::new(move || $body))
    };
}

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
