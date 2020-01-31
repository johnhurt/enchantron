use std::hash::{BuildHasherDefault, Hasher};

use super::{IPointHasher, ValueRect};
use crate::model::{IPoint, IRect, Point};

pub struct SinglePerlinGenerator<H: IPointHasher> {
    scale: u32,
    hasher: H,
    offset: IPoint,
}

impl<H> SinglePerlinGenerator<H>
where
    H: IPointHasher,
{
    pub fn new(
        scale: u32,
        offset: IPoint,
        hasher: H,
    ) -> SinglePerlinGenerator<H> {
        SinglePerlinGenerator {
            scale,
            hasher,
            offset,
        }
    }

    fn perlin_gradient(&self, position: &IPoint) -> Point {
        let hash = self.hasher.hash(position);

        // Take the right of the hash as the dx and the left as the dy
        let gx = ((hash as i32) as f64) / std::i32::MIN as f64;
        let gy = (((hash >> 32) as i32) as f64) / std::i32::MIN as f64;

        Point::new(gx, gy)
    }

    /// Precompute all the perlin gradients for all the node points that will
    /// be needed to compute the tiles in the given rectangle.
    fn get_perlin_gradients_for_rect(&self, rect: &IRect) {
        // offset the incoming rect to make it appear like the grid is lined
        // up with the origin. This just makes the computation easier.
        let offset_rect = IRect {
            top_left: &rect.top_left - &self.offset,
            size: rect.size.clone(),
        };

        let bounding_rect =
            self.get_bounding_rect_containing_rect(&offset_rect);
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

    /// Get the rectangle at the given octave containing the given point. This
    /// computation assumes that the point has been shiften to make the
    /// pernlin node grid line up with origin
    fn get_bounding_rect_at(&self, point: &IPoint) -> IRect {
        let side_size = self.scale as i64;

        IRect::new(
            point.x.div_euclid(side_size) * side_size,
            point.y.div_euclid(side_size) * side_size,
            side_size as usize,
            side_size as usize,
        )
    }

    /// Get the minimum bounding rectangle (where each corner is a perlin node)
    /// that contains the given rectangle. This computation assumes that the
    /// point has been shiften to make the pernlin node grid line up with origin
    fn get_bounding_rect_containing_rect(&self, rect: &IRect) -> IRect {
        let top_left_rect = self.get_bounding_rect_at(&rect.top_left);
        let bottom_right_rect = self.get_bounding_rect_at(&rect.bottom_right());

        let top_left = top_left_rect.top_left;
        let bottom_right = bottom_right_rect.bottom_right();
        let size = (bottom_right - &top_left)
            .to_size()
            .expect("Bad rect coords");

        IRect { top_left, size }
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

    pub fn get_rect(&self, rect: &IRect) -> ValueRect<f64> {
        let mut result = ValueRect::new(rect);

        result
    }
}

#[cfg(test)]
mod test {
    use super::super::DefaultXxHashIPointHasher;
    use super::*;

    fn new_generator() -> SinglePerlinGenerator<DefaultXxHashIPointHasher> {
        SinglePerlinGenerator::<DefaultXxHashIPointHasher>::new(
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
