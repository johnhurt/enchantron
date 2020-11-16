use super::{Entity, EntityData, EntityType, LocationKey};

#[derive(Debug, Clone, Copy, Hash, PartialEq)]
pub struct Player {
    pub entity: Entity,
    pub location_key: LocationKey,
}

impl From<&EntityData> for Player {
    fn from(entity_data: &EntityData) -> Player {
        debug_assert_eq!(entity_data.entity_type, EntityType::Player);

        Player {
            entity: entity_data.entity.expect("Expected player entity"),
            location_key: entity_data
                .location_key
                .expect("Player Entity Requires Location Key"),
        }
    }
}
