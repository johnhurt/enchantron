
use ui::{ HasSize, HasLocation };

pub struct TextureAtlasBox {
  width: i64,
  height: i64,
  left: i64,
  top: i64
}

impl TextureAtlasBox {
  pub fn new(width: i64,
      height: i64,
      left: i64,
      top: i64) -> TextureAtlasBox {
    TextureAtlasBox{
      width: width, 
      height: height, 
      left: left, 
      top: top
    }
  }
}

impl HasLocation for TextureAtlasBox {
  fn get_left(&self) -> i64 {
    self.left
  }

  fn get_top(&self) -> i64 {
    self.top
  }
}

impl HasSize for TextureAtlasBox {
  fn get_width(&self) -> i64 {
    self.width
  }

  fn get_height(&self) -> i64 {
    self.height
  }
}