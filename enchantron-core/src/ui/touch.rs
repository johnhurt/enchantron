use super::{Finger, RawTouch, RawTouchPoint, TouchPoint, ViewportInfo};
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
pub struct Touch {
    pub finger: Finger,
    pub start_time: Instant,
    pub point: TouchPoint,
    pub click_count: u8,
    pub move_dist_sqr_sum: f64,
}

impl Touch {
    pub fn new(
        finger: Finger,
        touch: &RawTouch,
        viewport_info: &ViewportInfo,
    ) -> Touch {
        let RawTouch {
            point, click_count, ..
        } = touch;

        Touch {
            finger,
            start_time: Instant::now(),
            point: TouchPoint::new(point, viewport_info),
            click_count: *click_count,
            move_dist_sqr_sum: 0.,
        }
    }

    /// Update this touch using the information in the given new touch, and
    /// return a copy of the result
    pub fn update(
        &mut self,
        new_touch: &RawTouch,
        viewport_info: &ViewportInfo,
    ) -> Touch {
        let RawTouch { point, .. } = new_touch;

        let dist_sqr_inc = point
            .global_point
            .distance_squared_to(&self.point.screen_point);
        self.move_dist_sqr_sum += dist_sqr_inc;
        self.point = TouchPoint::new(&new_touch.point, viewport_info);

        *self
    }
}
