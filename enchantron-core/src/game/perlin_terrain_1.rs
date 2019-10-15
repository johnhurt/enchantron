use super::{TerrainProvider, TerrainType};
use crate::model::IPoint;
use fnv::FnvHasher;
use std::hash::Hasher;

pub struct PerlineTerrain1 {}

impl PerlineTerrain1 {
    fn perlin_gradient(&self, octave: u32, position: &IPoint) -> (f64, f64) {
        let mut hasher = FnvHasher::default();

        hasher.write_i64(position.x);
        hasher.write_i64(position.y);

        let hash = hasher.finish();

        // Take the right of the hash as the dx and the left as the dy
        let dx = ((hash as i32) as f64) / std::i32::MIN as f64;
        let dy = ((hash as i32 >> 32) as f64) / std::i32::MIN as f64;

        (dx, dy)
    }

    fn lin_interp() -> f64 {}
}

impl TerrainProvider for PerlineTerrain1 {
    fn get_for(&self, position: &IPoint) -> TerrainType {}
}
