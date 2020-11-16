use crate::model::Point;

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct RawTouchPoint {
    pub global_point: Point,
    pub local_point: Point,
}

impl RawTouchPoint {
    pub fn new(
        global_x: f64,
        global_y: f64,
        local_x: f64,
        local_y: f64,
    ) -> RawTouchPoint {
        RawTouchPoint {
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
