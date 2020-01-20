use std::hash::{BuildHasher, BuildHasherDefault, Hash, Hasher};

use crate::model::{IPoint, IRect, Point};

pub struct SinglePerlinGenerator<H: Hasher + Clone + Default> {
    scale: u32,
    hasher: H,
    offset: IPoint,
}

impl<H> SinglePerlinGenerator<H>
where
    H: Hasher + Clone + Default,
{
    pub fn new(
        scale: u32,
        offset: IPoint,
        seed: u128,
    ) -> SinglePerlinGenerator<H> {
        SinglePerlinGenerator::inner_new(
            scale,
            offset,
            seed,
            BuildHasherDefault::<H>::default(),
        )
    }

    fn inner_new<B>(
        scale: u32,
        offset: IPoint,
        seed: u128,
        hasher_builer: B,
    ) -> SinglePerlinGenerator<H>
    where
        B: BuildHasher<Hasher = H>,
    {
        let mut hasher = hasher_builer.build_hasher();

        hasher.write_u128(seed);
        hasher.write_u32(scale);

        offset.hash(&mut hasher);

        SinglePerlinGenerator {
            offset,
            scale,
            hasher,
        }
    }

    fn perlin_gradient(&self, position: &IPoint) -> Point {
        let mut hasher = self.hasher.clone();

        position.hash(&mut hasher);

        let hash = hasher.finish();

        // Take the right of the hash as the dx and the left as the dy
        let gx = ((hash as i32) as f64) / std::i32::MIN as f64;
        let gy = (((hash >> 32) as i32) as f64) / std::i32::MIN as f64;

        Point::new(gx, gy)
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

    /// Get the rectangle at the given octave containing the given point
    fn get_bounding_rect_at(&self, point: &IPoint) -> IRect {
        let side_size = self.scale as i64;

        IRect::new(
            point.x.div_euclid(side_size) * side_size,
            point.y.div_euclid(side_size) * side_size,
            side_size as usize,
            side_size as usize,
        )
    }

    /// Get the perlin noise value at the given point
    pub fn get(&self, point: &IPoint) -> f64 {
        // The grid that defines the perlin points is offset, so shift the
        // given point in the opposite direction to make computation cleaner
        let offset_point = point - &self.offset;
        let bounding_rect = self.get_bounding_rect_at(&offset_point);

        let offset = self.proportional_difference(
            &bounding_rect.top_left,
            &offset_point,
            &bounding_rect.size.width,
        );

        // Calculate the dot gradients at each corner of the octave
        let (dg00, dg10, dg11, dg01) =
            bounding_rect.for_each_corner(|corner| {
                let gradient = self.perlin_gradient(corner);
                let delta = self.proportional_difference(
                    corner,
                    &offset_point,
                    &bounding_rect.size.width,
                );
                gradient.dot(&delta)
            });

        let ix0 = self.linear_interp(dg00, dg10, offset.x);
        let ix1 = self.linear_interp(dg01, dg11, offset.x);
        self.linear_interp(ix0, ix1, offset.y)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use twox_hash::XxHash64;

    fn new_generator() -> SinglePerlinGenerator<XxHash64> {
        SinglePerlinGenerator::<XxHash64>::new(
            16,
            Default::default(),
            Default::default(),
        )
    }

    #[test]
    fn test_proportional_difference() {
        let p = new_generator();

        assert_eq!(
            Point::new(0.125, -0.25),
            p.proportional_difference(
                &IPoint::new(100, 200),
                &IPoint::new(101, 198),
                &8usize
            )
        );
    }

    #[test]
    fn test_perlin_gradient() {
        let p = new_generator();

        for i in 30..40 {
            println!("{}\t{}", i, p.get(&IPoint::new(0, i)));
        }

        println!("blah");

        assert!(true);
    }
}
