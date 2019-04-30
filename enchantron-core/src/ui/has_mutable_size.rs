pub trait HasMutableSize {
    fn set_size_animated(
        &self,
        width: f64,
        height: f64,
        duraction_seconds: f64,
    );

    fn set_size(&self, width: f64, height: f64) {
        self.set_size_animated(width, height, 0.0);
    }
}
