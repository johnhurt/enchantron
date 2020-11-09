use super::{
    Entity, EntityData, EntityType, LocationKey, Player, SaveableLocation,
};
use crate::model::IRect;
use crate::presenter::PlayerPresenterState;
use one_way_slot_map::SlotMap;

pub struct SavedGame {
    pub(crate) seed: u64,
    pub(crate) elapsed_millis: u64,
    pub(crate) player: Player,
    pub(crate) entities: SlotMap<Entity, EntityType, EntityData>,
    pub(crate) locations: SlotMap<LocationKey, Entity, SaveableLocation>,
    pub(crate) player_presenter_states: Vec<(Entity, PlayerPresenterState)>,
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

        let player = {
            let to_update = entities.get_mut(&player_entity).unwrap();

            (*to_update).entity = Some(player_entity);
            (*to_update).location_key = Some(location_key);

            (to_update as &EntityData).into()
        };

        let player_presenter_states =
            vec![(player_entity, PlayerPresenterState::Spawning(0.))];

        SavedGame {
            seed,
            elapsed_millis: Default::default(),
            player,
            entities,
            locations,
            player_presenter_states,
        }
    }
}
