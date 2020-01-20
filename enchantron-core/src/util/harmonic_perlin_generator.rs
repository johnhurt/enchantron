use std::hash::{BuildHasherDefault, Hasher};

use super::SinglePerlinGenerator;

use crate::model::IPoint;

pub struct HarmonicPerlinGenerator<H: Hasher + Default + Clone> {
    harmonics: Vec<SinglePerlinGenerator<H>>,
}

impl<H: Hasher + Default + Clone> HarmonicPerlinGenerator<H> {
    pub fn new(
        root_scale: u32,
        root_offset: IPoint,
        multiplier: u8,
        offset_shift: IPoint,
        count: u8,
        seed: u128,
    ) -> HarmonicPerlinGenerator<H> {
        let mut result = HarmonicPerlinGenerator {
            harmonics: Vec::new(),
        };

        let mut scale = root_scale;
        let mut offset = root_offset;

        for _ in 0..count {
            result.harmonics.push(SinglePerlinGenerator::new(
                scale,
                offset.clone(),
                seed,
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
