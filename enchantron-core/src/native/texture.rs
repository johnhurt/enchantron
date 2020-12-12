use crate::ui::HasSize;

pub trait Texture: HasSize + Sync + Send + 'static {
    fn get_sub_texture(
        &self,
        left: f64,
        top: f64,
        width: f64,
        height: f64,
    ) -> Self;
}
