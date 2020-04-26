use super::SlotMapKeyData;
use std::convert::From;

pub trait SlotMapKey<T>: 'static + Copy + From<(T, SlotMapKeyData)> {
    fn get_slot_map_key_data(&self) -> &SlotMapKeyData;
}
