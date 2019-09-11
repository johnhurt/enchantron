use crate::model::Point;

pub trait HasMutableLocation {
    fn set_location_animated(&self, left: f64, top: f64, duration_seconds: f64);
    fn set_location(&self, left: f64, top: f64) {
        self.set_location_animated(left, top, 0.0)
    }

    fn set_location_point(&self, point: &Point) {
        self.set_location(point.x, point.y);
    }
}
