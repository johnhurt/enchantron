use std::os::raw::c_void;

use {ext_texture, get_ui_bindings};

use ui;
use ui::HasSize;

lazy_static! {
  static ref EXT_BINDING: ext_texture = get_ui_bindings().texture;
}

pub struct Texture(*mut c_void);

impl Texture {
  pub fn new(_self: *mut c_void) -> Texture {
    Texture(_self)
  }
}

impl HasSize for Texture {
  fn get_width(&self) -> i64 {
    (EXT_BINDING.get_width)(self.0)
  }

  fn get_height(&self) -> i64 {
    (EXT_BINDING.get_height)(self.0)
  }
}

impl ui::Texture for Texture {
  fn get_sub_texture(&self, left: i64, top: i64, width: i64, height: i64) -> Self {
    Texture::new((EXT_BINDING.get_sub_texture)(self.0, left, top, width, height))
  }
}

impl Drop for Texture {
  fn drop(&mut self) {
    (EXT_BINDING.destroy)(self.0)
  }
}