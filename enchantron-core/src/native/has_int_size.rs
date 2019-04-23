

pub trait HasIntSize {
  fn get_width(&self) -> i64;
  fn get_height(&self) -> i64;

  fn get_aspect_ratio(&self) -> f64 {
    (self.get_width() as f64) / (self.get_height() as f64)
  }
}