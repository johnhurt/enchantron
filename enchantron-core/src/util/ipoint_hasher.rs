use crate::model::IPoint;

pub trait IPointHasher {
    fn seed_u64(&mut self, seed: u64);
    fn seed_i64(&mut self, seed: i64);
    fn hash(&self, ipoint: &IPoint) -> u64;
}
