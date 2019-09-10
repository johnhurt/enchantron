use crate::model::{Point, Rect};

pub struct DragState {
    pub start_viewport_rect: Rect,
    pub start_point: Point,
}

impl DragState {
    pub fn new(start_point: Point, viewport_rect: Rect) -> DragState {
        DragState {
            start_viewport_rect: viewport_rect,
            start_point: start_point,
        }
    }
}
