use crate::model::Point;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DragPoint {
    drag_id: i64,
    global_point: Point,
    local_point: Point,
}

impl DragPoint {
    pub fn new(
        drag_id: i64,
        global_x: f64,
        global_y: f64,
        local_x: f64,
        local_y: f64,
    ) -> DragPoint {
        DragPoint {
            drag_id,
            global_point: Point {
                x: global_x,
                y: global_y,
            },
            local_point: Point {
                x: local_x,
                y: local_y,
            },
        }
    }
}
