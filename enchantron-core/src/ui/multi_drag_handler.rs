use super::{DragEventType, DragPoint};
use crate::event::DragEvent;

pub type DragFn = dyn Fn(DragEvent) + 'static + Send;

pub struct MultiDragHandler {
    multi_drag_fn: Box<DragFn>,
}

impl MultiDragHandler {
    pub fn new(
        drag_handler: impl Fn(DragEvent) + 'static + Send,
    ) -> MultiDragHandler {
        MultiDragHandler {
            multi_drag_fn: Box::new(drag_handler),
        }
    }

    pub fn on_one_drag_start(
        &self,
        drag_id: i64,
        global_x: f64,
        global_y: f64,
        local_x: f64,
        local_y: f64,
    ) {
        (self.multi_drag_fn)(DragEvent::new(
            DragEventType::Start,
            DragPoint::new(drag_id, global_x, global_y, local_x, local_y),
            None,
        ));
    }

    pub fn on_one_drag_move(
        &self,
        drag_id: i64,
        global_x: f64,
        global_y: f64,
        local_x: f64,
        local_y: f64,
    ) {
        (self.multi_drag_fn)(DragEvent::new(
            DragEventType::Move,
            DragPoint::new(drag_id, global_x, global_y, local_x, local_y),
            None,
        ));
    }

    pub fn on_one_drag_end(
        &self,
        drag_id: i64,
        global_x: f64,
        global_y: f64,
        local_x: f64,
        local_y: f64,
    ) {
        (self.multi_drag_fn)(DragEvent::new(
            DragEventType::End,
            DragPoint::new(drag_id, global_x, global_y, local_x, local_y),
            None,
        ));
    }

    #[allow(clippy::too_many_arguments)]
    pub fn on_two_drags_start(
        &self,
        drag_id_1: i64,
        global_x_1: f64,
        global_y_1: f64,
        local_x_1: f64,
        local_y_1: f64,
        drag_id_2: i64,
        global_x_2: f64,
        global_y_2: f64,
        local_x_2: f64,
        local_y_2: f64,
    ) {
        (self.multi_drag_fn)(DragEvent::new(
            DragEventType::Start,
            DragPoint::new(
                drag_id_1, global_x_1, global_y_1, local_x_1, local_y_1,
            ),
            Some(DragPoint::new(
                drag_id_2, global_x_2, global_y_2, local_x_2, local_y_2,
            )),
        ));
    }

    #[allow(clippy::too_many_arguments)]
    pub fn on_two_drags_move(
        &self,
        drag_id_1: i64,
        global_x_1: f64,
        global_y_1: f64,
        local_x_1: f64,
        local_y_1: f64,
        drag_id_2: i64,
        global_x_2: f64,
        global_y_2: f64,
        local_x_2: f64,
        local_y_2: f64,
    ) {
        (self.multi_drag_fn)(DragEvent::new(
            DragEventType::Move,
            DragPoint::new(
                drag_id_1, global_x_1, global_y_1, local_x_1, local_y_1,
            ),
            Some(DragPoint::new(
                drag_id_2, global_x_2, global_y_2, local_x_2, local_y_2,
            )),
        ));
    }

    #[allow(clippy::too_many_arguments)]
    pub fn on_two_drags_end(
        &self,
        drag_id_1: i64,
        global_x_1: f64,
        global_y_1: f64,
        local_x_1: f64,
        local_y_1: f64,
        drag_id_2: i64,
        global_x_2: f64,
        global_y_2: f64,
        local_x_2: f64,
        local_y_2: f64,
    ) {
        (self.multi_drag_fn)(DragEvent::new(
            DragEventType::End,
            DragPoint::new(
                drag_id_1, global_x_1, global_y_1, local_x_1, local_y_1,
            ),
            Some(DragPoint::new(
                drag_id_2, global_x_2, global_y_2, local_x_2, local_y_2,
            )),
        ));
    }
}
