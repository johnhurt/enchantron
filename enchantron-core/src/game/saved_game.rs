use super::{Entity, EntityData, EntityType, LocationKey, SaveableLocation};
use crate::model::IRect;
use crate::presenter::PlayerPresenterState;
use one_way_slot_map::SlotMap;

pub struct SavedGame {
    pub seed: u64,
    pub elapsed_millis: u64,
    pub entities: SlotMap<Entity, EntityType, EntityData>,
    pub locations: SlotMap<LocationKey, Entity, SaveableLocation>,
    pub player_presenter_states: Vec<(Entity, PlayerPresenterState)>,
}

impl SavedGame {
    pub fn new(seed: u64) -> SavedGame {
        let mut locations = SlotMap::new();
        let mut entities = SlotMap::new();
        let player_entity_data =
            EntityData::default_for_type(EntityType::Player);
        let player_entity: Entity =
            entities.insert(EntityType::Player, player_entity_data);

        let starting_location = IRect::new(0, 0, 1, 1);
        let stored_location =
            SaveableLocation::new(starting_location, player_entity);

        let location_key: LocationKey =
            locations.insert(player_entity, stored_location);

        {
            let to_update = entities.get_mut(&player_entity).unwrap();

            (*to_update).entity = Some(player_entity);
            (*to_update).location_key = Some(location_key);
        }

        let player_presenter_states =
            vec![(player_entity, PlayerPresenterState::Spawning(0.))];

        SavedGame {
            seed,
            elapsed_millis: Default::default(),
            entities,
            locations,
            player_presenter_states,
        }
    }
}
