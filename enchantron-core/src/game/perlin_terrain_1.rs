use super::{TerrainProvider, TerrainType};
use crate::model::{IPoint, IRect};
use crate::util::{HarmonicPerlinGenerator, RestrictedXxHasher, ValueRect};

const DEFAULT_OCTAVE_SCALE: u32 = 8;
const DEFAULT_OCTAVE_COUNT: u8 = 12;
const DEFAULT_MULTIPLIER: u8 = 2;
const DEAFULT_OFFSET: IPoint = IPoint { x: 0, y: 0 };
const DEFAULT_OFFSET_SHIFT: IPoint = IPoint { x: 6, y: 6 };

pub struct PerlinTerrain1 {
    generator: HarmonicPerlinGenerator<RestrictedXxHasher>,
}

impl Default for PerlinTerrain1 {
    fn default() -> PerlinTerrain1 {
        PerlinTerrain1::new(
            DEFAULT_OCTAVE_SCALE,
            DEAFULT_OFFSET,
            DEFAULT_MULTIPLIER,
            DEFAULT_OFFSET_SHIFT,
            DEFAULT_OCTAVE_COUNT,
            0,
        )
    }
}

impl PerlinTerrain1 {
    fn new(
        root_scale: u32,
        root_offset: IPoint,
        multiplier: u8,
        offset_shift: IPoint,
        count: u8,
        seed: u64,
    ) -> PerlinTerrain1 {
        PerlinTerrain1 {
            generator: HarmonicPerlinGenerator::<RestrictedXxHasher>::new(
                root_scale,
                root_offset,
                multiplier,
                offset_shift,
                count,
                seed,
            ),
        }
    }
}

impl TerrainProvider for PerlinTerrain1 {
    fn get_for(&self, position: &IPoint) -> TerrainType {
        let v = self.generator.get(position);

        if v < 0. {
            TerrainType::Dirt
        } else {
            TerrainType::Grass
        }
    }

    fn get_for_rect(&self, rect: &IRect) -> ValueRect<TerrainType> {
        self.generator.get_rect(rect).map(|v| {
            if *v < 0. {
                TerrainType::Dirt
            } else {
                TerrainType::Grass
            }
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::time::SystemTime;

    #[test]
    fn test_performance() {
        let gen = PerlinTerrain1::default();

        let rows = 128i64;
        let cols = 128i64;

        let row_start = rows * 3056;
        let col_start = cols * 10573;

        {
            let rect =
                IRect::new(col_start, row_start, cols as usize, rows as usize);

            let now = SystemTime::now();

            let values = gen.get_for_rect(&rect);

            println!("batch {:?}", now.elapsed());
        }

        {
            let now = SystemTime::now();

            for row in row_start..(row_start + rows) {
                for col in col_start..(col_start + cols) {
                    let point = IPoint::new(col, row);
                    gen.get_for(&point);
                }
            }

            println!("singles {:?}", now.elapsed());
        }

        //panic!("To get stdout");
    }
}
