
use super::{TerrainProvider, TerrainType};
use crate::model::{IPoint};
use fnv::FnvHasher;
use std::hash::Hasher;

pub struct PerlineTerrain1 {

}

impl TerrainProvider for PerlineTerrain1 {

    fn get_for(&self, position: &IPoint) -> TerrainType {
        let mut hasher = FnvHasher::default();

        hasher.write_i64(position.x);
        hasher.write_i64(position.y);

        let hash = hasher.finish();



    }

}