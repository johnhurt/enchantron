use super::{IPointHasher, SinglePerlinGenerator, ValueRect};

use crate::model::{IPoint, IRect};

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

            result
                .harmonics
                .push(SinglePerlinGenerator::new(scale, offset, hasher));
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

    pub fn get_rect(&self, rect: &IRect) -> ValueRect<f64> {
        let mut result =
            ValueRect::new_from_rect_with_defaults(rect.clone(), 1, 1);

        let mut working_space =
            ValueRect::new_from_rect_with_defaults(rect.clone(), 1, 1);

        for harmonic in &self.harmonics {
            harmonic.fill_rect(&mut working_space);

            result += &working_space;
        }

        result
    }
}

#[cfg(test)]
mod test {

    use super::super::RestrictedXxHasher;
    use super::*;

    #[test]
    fn test_repeatability() {
        let gen = HarmonicPerlinGenerator::<RestrictedXxHasher>::new(
            8,
            IPoint::new(3, 2),
            2,
            IPoint::new(-1, 3),
            16,
            0,
        );

        let terrain_rect = IRect::new(0, 0, 1, 1);

        let run_1 = gen.get_rect(&terrain_rect);

        for i in 0..10000 {
            let run_n = gen.get_rect(&terrain_rect).map(|a| a.clone());
            if !run_1.eq(&run_n) {
                println!("Got inconsistency on run {}", i);
            }
            assert_eq!(run_1, run_n);
        }
    }
}
