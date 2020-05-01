use super::GameEntity;
use crate::util::{SlotMapKey, SlotMapKeyData};

#[derive(Debug, Hash, Clone, Copy, PartialEq)]
pub struct GameEntitySlotKey {
    entity: GameEntity,
    slot_key: SlotMapKeyData,
}

impl SlotMapKey<GameEntity> for GameEntitySlotKey {
    fn get_slot_map_key_data(&self) -> &SlotMapKeyData {
        &self.slot_key
    }
}

impl From<(GameEntity, SlotMapKeyData)> for GameEntitySlotKey {
    fn from(f: (GameEntity, SlotMapKeyData)) -> Self {
        let (entity, slot_key) = f;
        GameEntitySlotKey { entity, slot_key }
    }
}
