use super::{TerrainProvider, TerrainType};
use crate::model::IPoint;
use fnv::FnvHasher;
use std::hash::Hasher;

const DEFAULT_OCTAVE_SCALE : u32 = 8;
const DEFAULT_OCTAVE_COUNT : u32 = 3;

pub struct PerlineTerrain1 {}

impl PerlineTerrain1 {
    fn perlin_gradient(&self, octave: u32, position: &IPoint) -> (f64, f64) {
        let mut hasher = FnvHasher::default();

        hasher.write_u32(octave);
        hasher.write_i64(position.x);
        hasher.write_i64(position.y);

        let hash = hasher.finish();

        // Take the right of the hash as the dx and the left as the dy
        let dx = ((hash as i32) as f64) / std::i32::MIN as f64;
        let dy = ((hash as i32 >> 32) as f64) / std::i32::MIN as f64;

        (dx, dy)
    }

    fn get_octave_top_left(&self, point: &IPoint, octave: u32)

    fn lin_interp(&self) -> f64 {}
}

impl TerrainProvider for PerlineTerrain1 {
    fn get_for(&self, position: &IPoint) -> TerrainType {}
}
