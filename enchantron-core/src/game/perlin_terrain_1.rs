use super::{TerrainProvider, TerrainType};
use crate::model::{IPoint, IRect, Point};

use std::hash::Hasher;

use cached::{cached, SizedCache};
use twox_hash::XxHash64;

const DEFAULT_OCTAVE_SCALE: u8 = 8;
const DEFAULT_OCTAVE_COUNT: u8 = 1;

// cached! {
//     PERLIN_VALUES: SizedCache<(u8,IPoint),Point>
//             = SizedCache::with_size(256);

    fn perlin_gradient(octave: u8, position: &IPoint) -> Point {
        let mut hasher = XxHash64::default();

        hasher.write_u8(octave);
        hasher.write_i64(position.x);
        hasher.write_i64(position.y);

        let hash = hasher.finish();

        // Take the right of the hash as the dx and the left as the dy
        let gx = ((hash as i32) as f64) / std::i32::MIN as f64;
        let gy = (((hash >> 32) as i32) as f64) / std::i32::MIN as f64;

        Point::new(gx, gy)
    }
//}

pub struct PerlinTerrain1 {
    octave_scale: u8,
    octave_count: u8,
}

impl Default for PerlinTerrain1 {
    fn default() -> PerlinTerrain1 {
        PerlinTerrain1::new(DEFAULT_OCTAVE_SCALE, DEFAULT_OCTAVE_COUNT)
    }
}

impl PerlinTerrain1 {
    fn new(octave_scale: u8, octave_count: u8) -> PerlinTerrain1 {
        PerlinTerrain1 {
            octave_scale: octave_scale,
            octave_count: octave_count,
        }
    }

    /// Get the rectangle at the given octave containing the given point
    fn fill_octave_rect_at(
        &self,
        target: &mut IRect,
        point: &IPoint,
        octave: u8,
    ) {
        let octave_side_size = (self.octave_scale as usize) << octave as usize;

        target.top_left.x = point.x.div_euclid(octave_side_size as i64) * octave_side_size as i64;
        target.top_left.y = point.y.div_euclid(octave_side_size as i64) * octave_side_size as i64;
        target.size.width = octave_side_size;
        target.size.height = octave_side_size;
    }

    /// Get the difference vector between the two given vectors (to minus from)
    /// scaled by the given unit length to produce floating point values for
    /// both x and y
    fn proportional_difference(
        &self,
        from: &IPoint,
        to: &IPoint,
        unit_length: &usize,
    ) -> Point {
        Point::new(
            (to.x - from.x) as f64 / *unit_length as f64,
            (to.y - from.y) as f64 / *unit_length as f64,
        )
    }

    /// Linear interpolation of a function f given its values at 0 and 1 and
    /// the propotional difference in between them
    fn linear_interp(&self, f_at_x0: f64, f_at_x1: f64, x: f64) -> f64 {
        (1. - x) * f_at_x0 + x * f_at_x1
    }

    /// Get the perlin noise value at the given point
    fn get(&self, point: &IPoint) -> f64 {
        let mut octave_rect = IRect::default();

        let mut result = 0.;

        for octave in 0..self.octave_count {
            self.fill_octave_rect_at(&mut octave_rect, point, octave);

            let offset = self.proportional_difference(
                &octave_rect.top_left,
                point,
                &octave_rect.size.width,
            );

            // Calculate the dot gradients at each corner of the octave
            let (dg00, dg10, dg11, dg01) =
                octave_rect.for_each_corner(|corner| {
                    let gradient = perlin_gradient(octave, corner);
                    let delta = self.proportional_difference(
                        corner,
                        point,
                        &octave_rect.size.width,
                    );
                    gradient.dot(&delta)
                });

            let ix0 = self.linear_interp(dg00, dg10, offset.x);
            let ix1 = self.linear_interp(dg01, dg11, offset.x);
            result += self.linear_interp(ix0, ix1, offset.y);
        }

        result
    }
}

impl TerrainProvider for PerlinTerrain1 {
    fn get_for(&self, position: &IPoint) -> TerrainType {
        let v = self.get(position);

        if v < 0. {
            TerrainType::Dirt
        } else {
            TerrainType::Grass
        }
    }
}

#[test]
fn test_proportional_difference() {

    let p = PerlinTerrain1::default();

    assert_eq!(Point::new(0.125, -0.25),
        p.proportional_difference(&IPoint::new(100,200), &IPoint::new(101, 198), &8usize));
}

#[test]
fn test_perlin_gradient() {

    let p = PerlinTerrain1::default();

    for i in 30..40 {
        println!("{}\t{}", i, p.get(&IPoint::new(0,i)));
    }

    println!("blah");

    assert!(true);
}