use crate::model::{IRect, ISize};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct TerrainUpdateInfo {
    pub zoom_level: usize,
    pub sprite_length_in_tiles: usize,
    pub terrain_rect: IRect,
    pub sprite_array_size: ISize,
}
