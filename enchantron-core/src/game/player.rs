use super::{EntityData, EntityType, LocationKey};

#[derive(Debug, Clone, Copy, Hash, PartialEq)]
pub struct Player {
    pub location_key: LocationKey,
}

impl From<EntityData> for Player {
    fn from(entity_data: EntityData) -> Player {
        debug_assert_eq!(entity_data.entity_type, EntityType::Player);

        Player {
            location_key: entity_data
                .location_key
                .expect("Player Entity Requires Location Key"),
        }
    }
}
