pub struct DragHandler {
    on_drag_start: Box<dyn Fn(f64, f64, f64, f64) + Send + 'static>,
    on_drag_move: Box<dyn Fn(f64, f64, f64, f64) + Send + 'static>,
    on_drag_end: Box<dyn Fn(f64, f64, f64, f64) + Send + 'static>,
}

impl DragHandler {
    pub fn new(
        on_drag_start: Box<dyn Fn(f64, f64, f64, f64) + Send + 'static>,
        on_drag_move: Box<dyn Fn(f64, f64, f64, f64) + Send + 'static>,
        on_drag_end: Box<dyn Fn(f64, f64, f64, f64) + Send + 'static>,
    ) -> DragHandler {
        DragHandler {
            on_drag_start,
            on_drag_move,
            on_drag_end,
        }
    }

    pub fn on_drag_start(
        &self,
        global_x: f64,
        global_y: f64,
        local_x: f64,
        local_y: f64,
    ) {
        (self.on_drag_start)(global_x, global_y, local_x, local_y);
    }

    pub fn on_drag_move(
        &self,
        global_x: f64,
        global_y: f64,
        local_x: f64,
        local_y: f64,
    ) {
        (self.on_drag_move)(global_x, global_y, local_x, local_y);
    }

    pub fn on_drag_end(
        &self,
        global_x: f64,
        global_y: f64,
        local_x: f64,
        local_y: f64,
    ) {
        (self.on_drag_end)(global_x, global_y, local_x, local_y);
    }
}

impl Drop for DragHandler {
    fn drop(&mut self) {
        println!("Dropping Drag Handler")
    }
}
