/// Trait for a type that can provide the current time in fractional seconds.
/// This is mostly for easier testing
pub trait TimeSource {
    fn current_time(&self) -> f64;
}
