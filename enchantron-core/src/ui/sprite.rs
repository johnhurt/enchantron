use native::Texture;

use ui::{
  HasMutableSize,
  HasMutableLocation,
  HasMutableVisibility,
  HasDragHandlers
};

pub trait Sprite
    : HasMutableSize
    + HasMutableLocation
    + HasMutableVisibility
    + HasDragHandlers
    + 'static {
  type T : Texture;

  fn set_texture(&self, texture: &Self::T);

  fn propagate_events_to(&self, &Self);

  fn remove_from_parent(&self);
}