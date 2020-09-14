macro_rules! create_magnify_handler {
    (
    on_magnify($scale_change_additive:ident, $zoom_center_x:ident, $zoom_center_y:ident) $magnify_body:block
  ) => {
        MagnifyHandler::new(Box::new(
            move |$scale_change_additive, $zoom_center_x, $zoom_center_y| {
                $magnify_body
            },
        ))
    };
}

pub struct MagnifyHandler {
    on_magnify: Box<dyn Fn(f64, f64, f64) + Send + 'static>,
}

impl MagnifyHandler {
    pub fn new(
        on_magnify: Box<dyn Fn(f64, f64, f64) + Send + 'static>,
    ) -> MagnifyHandler {
        MagnifyHandler { on_magnify }
    }

    pub fn on_magnify(
        &self,
        scale_change_additive: f64,
        zoom_center_x: f64,
        zoom_center_y: f64,
    ) {
        (self.on_magnify)(scale_change_additive, zoom_center_x, zoom_center_y);
    }
}

impl Drop for MagnifyHandler {
    fn drop(&mut self) {
        println!("Dropping Magnify Handler")
    }
}
