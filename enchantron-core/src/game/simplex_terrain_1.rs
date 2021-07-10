use crate::{
    model::{IPoint, IRect, Point},
    util::{SimplexGenerator, ValueRect},
};

use super::{TerrainProvider, TerrainType};

pub struct SimplexTerrain1 {
    gen: SimplexGenerator,
}

impl SimplexTerrain1 {
    pub fn new() -> SimplexTerrain1 {
        SimplexTerrain1 {
            gen: SimplexGenerator {},
        }
    }
}

impl Default for SimplexTerrain1 {
    fn default() -> Self {
        Self::new()
    }
}

impl TerrainProvider for SimplexTerrain1 {
    fn get_for(&self, position: &IPoint) -> TerrainType {
        let mut scaled_point = Point::new(position.x as f64, position.y as f64);

        scaled_point *= 1. / 647.0;

        let noise = self.gen.generate(scaled_point);

        if noise < 0_f32 {
            TerrainType::Dirt
        } else {
            TerrainType::Grass
        }
    }

    fn get_for_rect(&self, rect: &IRect) -> ValueRect<(f64, TerrainType)> {
        ValueRect::new_from_rect(*rect, 1, 1, |point| {
            (0.0, self.get_for(point))
        })
    }
}
