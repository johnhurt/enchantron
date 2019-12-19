use super::{HasSlotMapKeyData, SlotMapKey};

use anymap::AnyMap;

const CHUNK_SIZE: usize = 0xFFF_usize;

pub struct SlotMap {
    storage: AnyMap
}

