use crate::native::Texture;

use super::{
    HasDragHandlers, HasMutableLocation, HasMutableSize, HasMutableVisibility,
};

pub trait Sprite:
    HasMutableSize
    + HasMutableLocation
    + HasMutableVisibility
    + HasDragHandlers
    + 'static
{
    type T: Texture;

    fn set_texture(&self, texture: &Self::T);

    fn propagate_events_to(&self, event_target: &Self);

    fn remove_from_parent(&self);
}
