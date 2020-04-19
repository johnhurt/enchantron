use super::{HasSlotMapKeyData};

pub trait SlotMapKey: 'static + Copy + Default + HasSlotMapKeyData {}
