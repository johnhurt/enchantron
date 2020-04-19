use super::{HasSlotMapKeyData, SlotMapKey};

const CHUNK_SIZE: usize = 0xFFF_usize;

pub struct SlotMap {
    storage: AnyMap,
}
