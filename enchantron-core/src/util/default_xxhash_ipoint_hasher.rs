use super::IPointHasher;
use crate::model::IPoint;
use std::hash::{Hash, Hasher};
use twox_hash::XxHash64;

#[derive(Default)]
pub struct DefaultXxHashIPointHasher {
    hasher: XxHash64,
}

impl IPointHasher for DefaultXxHashIPointHasher {
    fn seed_u64(&mut self, seed: u64) {
        self.hasher.write_u64(seed);
    }

    fn seed_i64(&mut self, seed: i64) {
        self.hasher.write_i64(seed);
    }

    fn hash(&self, ipoint: &IPoint) -> u64 {
        let mut hasher = self.hasher.clone();
        ipoint.hash(&mut hasher);
        hasher.finish()
    }
}
