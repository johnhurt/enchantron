use super::EntityType;
use one_way_slot_map::{SlotMapKey, SlotMapKeyData};
use std::borrow::Borrow;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum Entity {
    Player(SlotMapKeyData),
}

impl SlotMapKey<EntityType> for Entity {}

impl From<(EntityType, SlotMapKeyData)> for Entity {
    fn from(params: (EntityType, SlotMapKeyData)) -> Entity {
        let (entity_type, key_Data) = params;

        match entity_type {
            EntityType::Player => Entity::Player(key_Data),
        }
    }
}

impl Borrow<SlotMapKeyData> for Entity {
    fn borrow(&self) -> &SlotMapKeyData {
        match self {
            Self::Player(slot_key) => slot_key,
        }
    }
}
