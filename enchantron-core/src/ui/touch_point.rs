use crate::model::{IPoint, Point};
use crate::ui::{RawTouchPoint, ViewportInfo};

#[derive(Debug, Default, Copy, Clone)]
pub struct TouchPoint {
    pub screen_point: Point,
    pub viewport_point: IPoint,
}

impl TouchPoint {
    pub fn new(
        raw_touch_point: &RawTouchPoint,
        viewport_info: &ViewportInfo,
    ) -> TouchPoint {
        TouchPoint {
            screen_point: raw_touch_point.global_point,
            viewport_point: viewport_info
                .get_terrain_tile_for(&raw_touch_point.global_point),
        }
    }
}
