use crate::native::Texture;

use super::{
    HasDragHandlers, HasMutableLocation, HasMutableSize, HasMutableVisibility,
    SpriteSink,
};

pub trait Sprite:
    HasMutableSize
    + HasMutableLocation
    + HasMutableVisibility
    + HasDragHandlers
    + Send
    + Sync
    + 'static
{
    type T: Texture;

    fn set_texture(&self, texture: &Self::T);

    fn propagate_events_to(&self, event_target: &Self);

    fn remove_from_parent(&self);

    fn set_parent(&self, parent: &Self);
}
