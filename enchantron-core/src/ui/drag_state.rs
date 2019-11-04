use crate::model::{Point, Rect};

pub struct DragState {
    pub last_drag_point: Point,
}

impl DragState {
    pub fn new(start_point: Point) -> DragState {
        DragState {
            last_drag_point: start_point,
        }
    }
}
