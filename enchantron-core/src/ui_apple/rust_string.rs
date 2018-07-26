
use ::ext_rust_string;

pub static EXT_BINDING : ext_rust_string = ext_rust_string {
  get_length: get_length,
  get_content: get_content,
  drop: drop
};

pub struct RustString(Vec<u8>);

impl RustString {
  pub fn new(real: String) -> RustString {
    RustString(real.into_bytes())
  }
}

impl Drop for RustString {
  fn drop(&mut self) {
    println!("Dropping Rust String");
  }
}

extern "C" fn get_length(string_ptr: *mut RustString) -> i64 {
  let s = unsafe { &*string_ptr };
  s.0.len() as i64
}

extern "C" fn get_content(string_ptr: *mut RustString) 
    -> *mut u8 {
  let s = unsafe { &*string_ptr };
  s.0.as_ptr() as *mut u8
}

extern "C" fn drop(string_ptr: *mut RustString) {
  let _ = unsafe { Box::from_raw(string_ptr) };
}