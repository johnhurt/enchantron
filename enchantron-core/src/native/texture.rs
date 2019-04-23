use native::HasIntSize;

pub trait Texture : HasIntSize + 'static {
  fn get_sub_texture(&self,
      left: i64,
      top: i64,
      width: i64,
      height: i64)
          -> Self;
}