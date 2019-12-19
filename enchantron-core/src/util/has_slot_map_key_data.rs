use super::SlotMapKeyData;

const CHUNK_ID_BIT_MASK: u64 = 0xFFFF_FFFF_0000_0000u64;
const CHUNK_ID_BIT_SHIFT: u8 = 64u8 - 24u8;

const INDEX_IN_CHUNK_MASK: u64 = 0x0000_0000_FF00_0000u64;
const INDEX_IN_CHUNK_SHIFT: u8 = 64u8 - 32u8;

const GENERATION_MASK: u64 = 0x0000_0000_00FF_FFFFu64;

pub trait HasSlotMapKeyData: 'static {
    fn get_slot_map_key_data(&self) -> &SlotMapKeyData;

    fn get_chunk_id(&self) -> usize {
        ((CHUNK_ID_BIT_MASK & self.get_slot_map_key_data())
            >> CHUNK_ID_BIT_SHIFT) as usize
    }

    fn get_index_in_chunk(&self) -> usize {
        ((INDEX_IN_CHUNK_MASK & self.get_slot_map_key_data())
            >> INDEX_IN_CHUNK_SHIFT) as usize
    }

    fn get_generation(&self) -> usize {
        (GENERATION_MASK & self.get_slot_map_key_data()) as usize
    }
}
