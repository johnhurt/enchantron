macro_rules! create_magnify_handler {
    (
    on_magnify($scale_change_additive:ident) $magnify_body:block
  ) => {
        MagnifyHandler::new(Box::new(move |$scale_change_additive| {
            $magnify_body
        }))
    };
}

pub struct MagnifyHandler {
    on_magnify: Box<dyn Fn(f64) + 'static>,
}

impl MagnifyHandler {
    pub fn new(on_magnify: Box<dyn Fn(f64) + 'static>) -> MagnifyHandler {
        MagnifyHandler {
            on_magnify: on_magnify,
        }
    }

    pub fn on_magnify(&self, scale_change_additive: f64) {
        (self.on_magnify)(scale_change_additive);
    }
}

impl Drop for MagnifyHandler {
    fn drop(&mut self) {
        println!("Dropping Magnify Handler")
    }
}
