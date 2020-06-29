use super::HasIntSize;

pub trait Texture: HasIntSize + Sync + Send + 'static {
    fn get_sub_texture(
        &self,
        left: i64,
        top: i64,
        width: i64,
        height: i64,
    ) -> Self;

    fn set_center_registration(&self, center_registered: bool);
}
