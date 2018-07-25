
use ::ext_rust_string;

pub static EXT_BINDING : ext_rust_string = ext_rust_string {
  new: new_rust_string,
  get_length: get_rust_string_length,
  get_content: get_rust_string_content
};

pub struct RustString(Vec<u8>);

impl RustString {
  pub fn new(real: String) -> RustString {
    RustString(real.into_bytes())
  }

  pub fn to_string(self) -> String {
    String::from_utf8(self.0).unwrap()
  }
}

pub extern "C" fn new_rust_string(length: i64) -> *mut RustString {
  let mut result = RustString(Vec::with_capacity(length as usize));
  unsafe { result.0.set_len(length as usize); }
  Box::into_raw(Box::new(result))
}

pub extern "C" fn get_rust_string_length(string_ptr: *mut RustString) -> i64 {
  let s = unsafe { &*string_ptr };
  s.0.len() as i64
}

pub extern "C" fn get_rust_string_content(string_ptr: *mut RustString) 
    -> *mut u8 {
  let s = unsafe { &*string_ptr };
  s.0.as_ptr() as *mut u8
}
