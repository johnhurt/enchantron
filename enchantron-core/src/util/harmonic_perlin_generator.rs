use super::{IPointHasher, SinglePerlinGenerator};

use crate::model::IPoint;

pub struct HarmonicPerlinGenerator<H: IPointHasher + Default> {
    harmonics: Vec<SinglePerlinGenerator<H>>,
}

impl<H: IPointHasher + Default> HarmonicPerlinGenerator<H> {
    pub fn new(
        root_scale: u32,
        root_offset: IPoint,
        multiplier: u8,
        offset_shift: IPoint,
        count: u8,
        seed: u64,
    ) -> HarmonicPerlinGenerator<H> {
        let mut result = HarmonicPerlinGenerator {
            harmonics: Vec::new(),
        };

        let mut scale = root_scale;
        let mut offset = root_offset;

        for _ in 0..count {
            let mut hasher = H::default();

            hasher.seed_u64(seed);
            hasher.seed_u64(scale as u64);
            hasher.seed_i64(offset.x);
            hasher.seed_i64(offset.y);

            result.harmonics.push(SinglePerlinGenerator::new(
                scale,
                offset.clone(),
                hasher,
            ));
            scale *= multiplier as u32;
            offset += &offset_shift;
        }

        result
    }

    pub fn get(&self, point: &IPoint) -> f64 {
        let mut result = 0.;

        for harmonic in &self.harmonics {
            result += harmonic.get(point);
        }

        result
    }
}
