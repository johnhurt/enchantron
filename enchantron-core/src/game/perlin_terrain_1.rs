use super::{TerrainProvider, TerrainType};
use crate::model::{IPoint, Point};
use crate::util::HarmonicPerlinGenerator;

use twox_hash::XxHash64;

const DEFAULT_OCTAVE_SCALE: u32 = 8;
const DEFAULT_OCTAVE_COUNT: u8 = 12;
const DEFAULT_MULTIPLIER: u8 = 2;
const DEAFULT_OFFSET: IPoint = IPoint { x: 0, y: 0 };
const DEFAULT_OFFSET_SHIFT: IPoint = IPoint { x: 6, y: 6 };

pub struct PerlinTerrain1 {
    generator: HarmonicPerlinGenerator<XxHash64>,
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
        seed: u128,
    ) -> PerlinTerrain1 {
        PerlinTerrain1 {
            generator: HarmonicPerlinGenerator::<XxHash64>::new(
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
}
