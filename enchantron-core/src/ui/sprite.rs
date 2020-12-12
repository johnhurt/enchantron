use crate::native::{Animation, Texture};

use super::{
    Color, HasMutableColor, HasMutableLocation, HasMutableSize,
    HasMutableVisibility, HasMutableZLevel,
};

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
    type C: Color;

    fn set_texture(&self, texture: &Self::T);

    fn remove_from_parent(&self);

    fn animate(&self, animation: &Self::A, frame_duration_sec: f64);

    fn clear_animations(&self);
}
