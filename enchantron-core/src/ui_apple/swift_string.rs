use std::os::raw::c_void;

use ::{ext_swift_string, get_ui_bindings};

pub struct SwiftString(*mut c_void);

lazy_static! {
  static ref EXT_BINDING: ext_swift_string = get_ui_bindings().swift_string;
}

impl SwiftString {

  pub fn new(_self: *mut c_void) -> SwiftString {
    SwiftString(_self)
  }

  fn get_length(&self) -> i64 {
    (EXT_BINDING.get_length)(self.0)
  }

  fn get_content(&self) -> *mut u8 {
    (EXT_BINDING.get_content)(self.0)
  }

  pub fn to_string(&self) -> String {
    let length = self.get_length() as usize;
    let mut vec_data : Vec<u8> = Vec::with_capacity(length);
    unsafe {
      vec_data.set_len(length);
      self.get_content().copy_to_nonoverlapping(
          vec_data.as_mut_ptr(), 
          length);
    }

    String::from_utf8(vec_data).unwrap()
  }
}

impl Drop for SwiftString {
  fn drop(&mut self) {
    (EXT_BINDING.destroy)(self.0)
  }
}