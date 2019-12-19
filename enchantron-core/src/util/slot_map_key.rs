use super::{HasSlotMapKeyData, SlotMapKeyData};

pub trait SlotMapKey: 'static + Clone + HasSlotMapKeyData {}
