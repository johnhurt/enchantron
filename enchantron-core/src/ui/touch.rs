use super::{DragPoint, Finger, RawTouch};
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
pub struct Touch {
    pub finger: Finger,
    pub start_time: Instant,
    pub point: DragPoint,
    pub click_count: u8,
    pub move_dist_sqr_sum: f64,
}

impl Touch {
    pub fn new(finger: Finger, touch: RawTouch) -> Touch {
        let RawTouch {
            point, click_count, ..
        } = touch;

        Touch {
            finger,
            start_time: Instant::now(),
            point,
            click_count,
            move_dist_sqr_sum: 0.,
        }
    }

    /// Update this touch using the information in the given new touch, and
    /// return a copy of the result
    pub fn update(&mut self, new_touch: RawTouch) -> Touch {
        let RawTouch { point, .. } = new_touch;

        let dist_sqr_inc = point
            .global_point
            .distance_squared_to(&self.point.global_point);
        self.move_dist_sqr_sum += dist_sqr_inc;
        self.point = point;

        *self
    }
}
