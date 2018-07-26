use std::os::raw::c_void;

use {ext_texture_loader, get_native_bindings};

use native;

use ui_impl::{Texture, RustString};

lazy_static! {
  static ref EXT_BINDING : ext_texture_loader = get_native_bindings().texture_loader;
}

pub struct TextureLoader(*mut c_void);

impl TextureLoader {
  pub fn new(_self: *mut c_void) -> TextureLoader {
    TextureLoader(_self)
  }
}

impl native::TextureLoader for TextureLoader {
  type Tex = Texture;

  fn load_texture(&self, resource_name: String) -> Self::Tex  {
    let text = RustString::new(resource_name);
    let text_ptr = Box::into_raw(Box::new(text));

    Texture::new((EXT_BINDING.load_texture)(self.0, text_ptr))
  }
}
