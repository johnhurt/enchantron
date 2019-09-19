use crate::model::Point;

pub trait HasMutableScale {
    fn set_scale_and_location(
        &self,
        scale: f64,
        top_left_x: f64,
        top_left_y: f64,
    );

    fn set_scale_and_location_point(&self, scale: f64, top_left: &Point) {
        self.set_scale_and_location(scale, top_left.x, top_left.y);
    }
}
