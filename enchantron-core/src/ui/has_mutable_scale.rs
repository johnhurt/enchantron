pub trait HasMutableScale {
    fn set_scale_animated(&self, scale: f64, duraction_seconds: f64);

    fn set_scale(&self, scale: f64) {
        self.set_scale_animated(scale, 0.);
    }
}
