use super::{Entity, EntityType, LocationKey};
use evmap::ShallowCopy;
use std::mem::ManuallyDrop;

/// Internal struct for keeping track of an entities aspect keys and message
/// channel
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityData {
    pub entity_type: EntityType,
    pub entity: Option<Entity>,
    pub location_key: Option<LocationKey>,
}

impl EntityData {
    pub fn default_for_type(entity_type: EntityType) -> EntityData {
        EntityData {
            entity_type,
            entity: Default::default(),
            location_key: Default::default(),
        }
    }
}

/// ShallowCopy for EntityData is a regular copy since EntityData derives Copy
impl ShallowCopy for EntityData {
    unsafe fn shallow_copy(&self) -> ManuallyDrop<Self> {
        ManuallyDrop::new(*self)
    }
}
