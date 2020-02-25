use std::hash::Hasher;

use super::{IPointHasher, ValueRect};
use crate::model::{IPoint, IRect, ISize, Point};

pub struct SinglePerlinGenerator<H: IPointHasher> {
    scale: u32,
    hasher: H,
    offset: IPoint,
}

#[derive(Clone, Default, Debug)]
struct PerlinGradientCoefs {
    dx_coef: f64,
    dy_coef: f64,
    dx_2_coef: f64,
    dy_2_coef: f64,
    dx_dy_coef: f64,
    dx_2_dy_coef: f64,
    dx_dy_2_coef: f64,
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
    fn get_perlin_gradients_ceofs_for_rect(
        &self,
        rect: &IRect,
    ) -> ValueRect<PerlinGradientCoefs> {
        let bounding_rect = self.get_bounding_rect_containing_rect(rect);

        let stride = self.scale as usize;

        let perlin_rect_count_x = &bounding_rect.size.width / stride;
        let perlin_rect_count_y = &bounding_rect.size.height / stride;

        // We need an extra perlin node value on the bottom and right of the
        // bounding rect to make sure for each tile we have the 4 corners in
        // the box on the perlin grid containing it
        let perlin_node_count_x = perlin_rect_count_x + 1;
        let perlin_node_count_y = perlin_rect_count_y + 1;

        let mut gradients =
            ValueRect::<Point>::new_from_point_and_strides_with_defaults(
                bounding_rect.top_left,
                stride,
                stride,
                perlin_node_count_x,
                perlin_node_count_y,
            );

        gradients.for_each_mut(|point, value| {
            *value = self.perlin_gradient(point);
        });

        // Now generate the perlin coeficients based on the computed gradients
        let mut result =
            ValueRect::<PerlinGradientCoefs>::new_from_point_and_strides_with_defaults(
                gradients.rect().top_left.clone(),
                stride,
                stride,
                perlin_rect_count_x,
                perlin_rect_count_y,
            );

        let one_over_scale = 1.0 / (self.scale as f64);
        let one_over_scale_2 = one_over_scale * one_over_scale;
        let one_over_scale_3 = one_over_scale * one_over_scale * one_over_scale;

        result.get_raw_values_mut().iter_mut().enumerate().for_each(
            |(index, coefs)| {
                let row = index / perlin_rect_count_x;
                let col = index % perlin_rect_count_x;

                let ref g_tl = gradients.get(col, row).unwrap();
                let ref g_tr = gradients.get(col + 1, row).unwrap();
                let ref g_br = gradients.get(col + 1, row + 1).unwrap();
                let ref g_bl = gradients.get(col, row + 1).unwrap();

                coefs.dx_coef = one_over_scale * (g_tl.x - g_tr.x);
                coefs.dy_coef = one_over_scale * (g_tl.y - g_bl.y);
                coefs.dx_2_coef = one_over_scale_2 * (g_tr.x - g_tl.x);
                coefs.dy_2_coef = one_over_scale_2 * (g_bl.y - g_tl.y);
                coefs.dx_dy_coef = one_over_scale_2
                    * (g_tr.x + g_tr.y + g_bl.x + g_bl.y
                        - (g_tl.x + g_tl.y + g_br.x + g_br.y));
                coefs.dx_2_dy_coef =
                    &one_over_scale_3 * (g_tl.x + g_br.x - (g_tr.x + g_bl.x));
                coefs.dx_dy_2_coef =
                    one_over_scale_3 * (g_tl.y + g_br.y - (g_tr.y + g_bl.y));
            },
        );

        result
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

    /// Get the rectangle at the given octave containing the given point.
    fn get_bounding_rect_at(&self, point: &IPoint) -> IRect {
        let side_size = self.scale as i64;

        IRect::new(
            (&point.x - &self.offset.x).div_euclid(side_size) * side_size
                + &self.offset.x,
            (&point.y - &self.offset.y).div_euclid(side_size) * side_size
                + &self.offset.y,
            side_size as usize,
            side_size as usize,
        )
    }

    /// Get the minimum bounding rectangle (where each corner is a perlin node)
    /// that contains the given rectangle.
    fn get_bounding_rect_containing_rect(&self, rect: &IRect) -> IRect {
        let top_left_rect = self.get_bounding_rect_at(&rect.top_left);
        let bottom_right_point = rect.bottom_right_exclusive();
        let bottom_right_rect = self.get_bounding_rect_at(&bottom_right_point);

        let top_left = top_left_rect.top_left;
        let bottom_right = bottom_right_rect.bottom_right_exclusive();
        let size = (bottom_right - &top_left)
            .to_size()
            .expect("Bad rect coords");

        IRect { top_left, size }
    }

    /// Calculate the perlin noise value for the given set of perlin gradient
    /// corners and point and perlin-node top left
    fn calculate_perlin_value(
        &self,
        top_left_gradient: &Point,
        top_right_gradient: &Point,
        bottom_right_gradient: &Point,
        bottom_left_gradient: &Point,
        perlin_node_top_left: &IPoint,
        point: &IPoint,
    ) -> f64 {
        // This will be a vector somewhere in the square { [0, 1), [0, 1) }
        let mut scaled_offset = Point::new(
            (&point.x - &perlin_node_top_left.x) as f64 / self.scale as f64,
            (&point.y - &perlin_node_top_left.y) as f64 / self.scale as f64,
        );

        let dg_top_left = top_left_gradient.dot(&scaled_offset);

        // move the scaled offset to being from the top right, and dot the
        // top-right gradient dotted with the result
        scaled_offset.x = scaled_offset.x - 1.;
        let dg_top_right = top_right_gradient.dot(&scaled_offset);

        // move the scaled offset to being from the bottom right, and dot the
        // bottom-right gradient dotted with the result
        scaled_offset.y = scaled_offset.y - 1.;
        let dg_bottom_right = bottom_right_gradient.dot(&scaled_offset);

        // move the scaled offset to being from the bottom left, and dot the
        // bottom-left gradient dotted with the result
        scaled_offset.x = scaled_offset.x + 1.;
        let dg_bottom_left = bottom_left_gradient.dot(&scaled_offset);

        // Bring the scaled offset back to being referenced off the top left,
        // and perform the linear interpolation to get the final result
        scaled_offset.y = scaled_offset.y + 1.;

        let ix0 =
            self.linear_interp(dg_top_left, dg_top_right, scaled_offset.x);
        let ix1 = self.linear_interp(
            dg_bottom_left,
            dg_bottom_right,
            scaled_offset.x,
        );
        self.linear_interp(ix0, ix1, scaled_offset.y)
    }

    /// Get the perlin noise value at the given point
    pub fn get(&self, point: &IPoint) -> f64 {
        let bounding_rect = self.get_bounding_rect_at(point);

        let offset = self.proportional_difference(
            &bounding_rect.top_left,
            point,
            &bounding_rect.size.width,
        );

        // Calculate the dot gradients at each corner of the octave
        let (dg00, dg10, dg11, dg01) =
            bounding_rect.for_each_corner(|corner| {
                let gradient = self.perlin_gradient(corner);
                let delta = self.proportional_difference(
                    corner,
                    &point,
                    &bounding_rect.size.width,
                );

                gradient.dot(&delta)
            });

        let ix0 = self.linear_interp(dg00, dg10, offset.x);
        let ix1 = self.linear_interp(dg01, dg11, offset.x);
        self.linear_interp(ix0, ix1, offset.y)
    }

    pub fn get_rect(&self, rect: &IRect) -> ValueRect<f64> {
        let mut result =
            ValueRect::new_from_rect_with_defaults(rect.clone(), 1, 1);
        self.fill_rect(&mut result);
        result
    }

    /// Get a filled rectangle of perlin values
    pub fn fill_rect(&self, target: &mut ValueRect<f64>) {
        let perlin_gradient_coefs =
            self.get_perlin_gradients_ceofs_for_rect(target.rect());

        let first_perlin_rect = IRect {
            top_left: perlin_gradient_coefs.rect().top_left.clone(),
            size: ISize::new(self.scale as usize, self.scale as usize),
        };

        let mut coefs: &PerlinGradientCoefs =
            perlin_gradient_coefs.get(0, 0).unwrap();

        let left_most_x = first_perlin_rect.top_left.x;

        let scale = self.scale as i64;
        let scale_minus_one = scale - 1;
        let mut curr_perlin_rect_top_left_x = first_perlin_rect.top_left.x;
        let mut curr_perlin_rect_top_left_y = first_perlin_rect.top_left.y;
        let mut max_x = curr_perlin_rect_top_left_x + scale_minus_one;
        let mut max_y = curr_perlin_rect_top_left_y + scale_minus_one;
        let mut idx_x = 0usize;
        let mut idx_y = 0usize;
        let mut dx = 0f64;
        let mut dy = 0f64;

        target.for_each_mut(|point, value| {
            if point.y > max_y {
                curr_perlin_rect_top_left_x = left_most_x;
                curr_perlin_rect_top_left_y += scale;
                idx_x = 0;
                idx_y += 1;
                max_x = curr_perlin_rect_top_left_x + scale_minus_one;
                max_y = curr_perlin_rect_top_left_y + scale_minus_one;
                coefs = perlin_gradient_coefs.get(idx_x, idx_y).unwrap();
            } else if point.x < curr_perlin_rect_top_left_x {
                curr_perlin_rect_top_left_x = left_most_x;
                idx_x = 0;
                max_x = curr_perlin_rect_top_left_x + scale_minus_one;
                coefs = perlin_gradient_coefs.get(idx_x, idx_y).unwrap();
            } else if point.x > max_x {
                curr_perlin_rect_top_left_x += scale;
                idx_x += 1;
                max_x = curr_perlin_rect_top_left_x + scale_minus_one;
                coefs = perlin_gradient_coefs.get(idx_x, idx_y).unwrap();
            }

            dx = (point.x - curr_perlin_rect_top_left_x) as f64;
            dy = (point.y - curr_perlin_rect_top_left_y) as f64;

            *value = (coefs.dx_coef + dx * coefs.dx_2_coef) * dx
                + (coefs.dy_coef + dy * coefs.dy_2_coef) * dy
                + (coefs.dx_dy_coef
                    + coefs.dx_2_dy_coef * dx
                    + coefs.dx_dy_2_coef * dy)
                    * dx
                    * dy;
        });
    }
}

#[cfg(test)]
mod test {
    use super::super::RestrictedXxHasher;
    use super::*;

    fn default_generator() -> SinglePerlinGenerator<RestrictedXxHasher> {
        new_generator(0, 16, 0, 0)
    }

    fn new_generator(
        seed: u64,
        scale: u32,
        offset_x: i64,
        offset_y: i64,
    ) -> SinglePerlinGenerator<RestrictedXxHasher> {
        let mut hasher = RestrictedXxHasher::default();
        hasher.seed_u64(seed);
        hasher.seed_u64(scale as u64);
        hasher.seed_i64(offset_x);
        hasher.seed_i64(offset_y);

        SinglePerlinGenerator::<RestrictedXxHasher>::new(
            scale,
            IPoint::new(offset_x, offset_y),
            hasher,
        )
    }

    #[test]
    fn test_proportional_difference() {
        let p = default_generator();

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
    fn test_get_bounding_rect() {
        let point = IPoint::new(21, -34);
        let p = default_generator();

        let bounding_rect = p.get_bounding_rect_at(&point);

        let expected = IRect::new(16, -48, 16, 16);
        assert_eq!(expected, bounding_rect);
    }

    #[test]
    fn test_get_bounding_rect_containing_rect() {
        let rect = IRect::new(21, -34, 33, 67);
        let p = default_generator();

        let bounding_rect = p.get_bounding_rect_containing_rect(&rect);

        let expected = IRect::new(16, -48, 48, 96);
        assert_eq!(expected, bounding_rect);
    }

    #[test]
    fn test_single_vs_rect() {
        let p = new_generator(123, 16, 6, 9);
        let rect = IRect::new(100, -200, 20, 45);
        let values = p.get_rect(&rect);

        assert_eq!(&20, values.values_width());
        assert_eq!(&45, values.values_height());

        for col in 0..20usize {
            for row in 0..45usize {
                let point =
                    IPoint::new(100i64 + col as i64, -200i64 + row as i64);
                let expected = Some(p.get(&point));
                let value = values.get(col, row).cloned();
                if expected != value {
                    println!("{}, {}", col, row);
                }
                assert_eq!(expected, value);
            }
        }
    }

    #[test]
    fn test_repeatablity() {
        let gen = new_generator(123, 16, 6, 9);
        let terrain_rect = IRect::new(0, 0, 4, 4);

        let run_1 = gen.get_rect(&terrain_rect);

        for i in 0..10000 {
            let run_n = gen.get_rect(&terrain_rect);
            if !run_1.eq(&run_n) {
                println!("Got inconsistency on run {}", i);
            }
            assert_eq!(run_1, run_n);
        }
    }
}
