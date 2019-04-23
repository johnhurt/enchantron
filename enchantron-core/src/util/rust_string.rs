
pub struct RustString(Vec<u8>);

impl RustString {
  pub fn new(real: String) -> RustString {
    RustString(real.into_bytes())
  }
}

impl RustString {
  pub fn get_length(&self) -> i64 {
    self.0.len() as i64
  }

  pub fn get_content(&self) -> *mut u8 {
    self.0.as_ptr() as *mut u8
  }
}
