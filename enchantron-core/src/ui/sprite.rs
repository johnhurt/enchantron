use super::{
    Color, HasMutableColor, HasMutableLocation, HasMutableSize,
    HasMutableVisibility, HasMutableZLevel,
};
use crate::model::Rect;
use crate::native::{Animation, Texture};

pub trait Sprite:
    HasMutableSize
    + HasMutableLocation
    + HasMutableVisibility
    + HasMutableZLevel
    + HasMutableColor
    + Send
    + Sync
    + Unpin
    + 'static
{
    type T: Texture;
    type A: Animation;

    fn set_rect(&self, rect: &Rect) {
        self.set_location_point(&rect.top_left);
        self.set_size(rect.size.width, rect.size.height);
    }

    fn set_texture(&self, texture: &Self::T);

    fn remove_from_parent(&self);

    fn animate(&self, animation: &Self::A, frame_duration_sec: f64);

    fn clear_animations(&self);
}
