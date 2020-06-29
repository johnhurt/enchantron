use super::GameEntity;
use one_way_slot_map::define_key_type;

define_key_type!(pub LocationKey<GameEntity> : Copy + Clone + Debug + PartialEq + Eq + Hash);
