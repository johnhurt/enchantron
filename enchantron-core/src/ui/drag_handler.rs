macro_rules! create_drag_handler {
    (
    on_drag_start($start_global_x:ident, $start_global_y:ident, $start_local_x:ident, $start_local_y:ident) $start_body:block ,
    on_drag_move($move_global_x:ident, $move_global_y:ident, $move_local_x:ident, $move_local_y:ident) $move_body:block ,
    on_drag_end($end_global_x:ident, $end_global_y:ident, $end_local_x:ident, $end_local_y:ident) $end_body:block
  ) => {
        DragHandler::new(
            Box::new(
                move |$start_global_x,
                      $start_global_y,
                      $start_local_x,
                      $start_local_y| $start_body,
            ),
            Box::new(
                move |$move_global_x,
                      $move_global_y,
                      $move_local_x,
                      $move_local_y| $move_body,
            ),
            Box::new(
                move |$end_global_x,
                      $end_global_y,
                      $end_local_x,
                      $end_local_y| $end_body,
            ),
        )
    };
}

pub struct DragHandler {
    on_drag_start: Box<Fn(f64, f64, f64, f64) + 'static>,
    on_drag_move: Box<Fn(f64, f64, f64, f64) + 'static>,
    on_drag_end: Box<Fn(f64, f64, f64, f64) + 'static>,
}

impl DragHandler {
    pub fn new(
        on_drag_start: Box<Fn(f64, f64, f64, f64) + 'static>,
        on_drag_move: Box<Fn(f64, f64, f64, f64) + 'static>,
        on_drag_end: Box<Fn(f64, f64, f64, f64) + 'static>,
    ) -> DragHandler {
        DragHandler {
            on_drag_start: on_drag_start,
            on_drag_move: on_drag_move,
            on_drag_end: on_drag_end,
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
