use super::{IPointHasher, SinglePerlinGenerator, ValueRect};

use crate::model::{IPoint, IRect};

use std::time::SystemTime;

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

    pub fn get_rect(&self, rect: &IRect) -> ValueRect<f64> {
        let mut now = SystemTime::now();

        let mut result =
            ValueRect::new_from_rect_with_defaults(rect.clone(), 1, 1);

        let mut working_space =
            ValueRect::new_from_rect_with_defaults(rect.clone(), 1, 1);

        println!(
            "time to create result and working space {:?}",
            now.elapsed()
        );
        let mut i = 0;

        for harmonic in &self.harmonics {
            now = SystemTime::now();

            harmonic.fill_rect(&mut working_space);

            println!("time to create perlin {} - {:?}", i, now.elapsed());

            now = SystemTime::now();

            result += &working_space;

            println!(
                "time to add perlin {} to result - {:?}",
                i,
                now.elapsed()
            );

            i += 1;
        }

        result
    }
}
