use super::Entity;
use one_way_slot_map::define_key_type;

define_key_type!(pub LocationKey<Entity> : Copy + Clone + Debug + PartialEq + Eq + Hash);
