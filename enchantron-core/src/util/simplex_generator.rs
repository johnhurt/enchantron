use crate::model::{IPoint, Point};

const INV_SQRT_2: f32 = 1.;

const CX: f32 = 0.21132487; // (3.0-sqrt(3.0))/6.0
const CY: f32 = 0.3660254; // 0.5 * (sqrt(3.0)-1.0)
const CZ: f32 = -0.57735026; // -1 + 2 * CX
const CW: f32 = 0.024390243; // 1.0/41.0

type Vec3 = [f32; 3];
type Vec2 = [f32; 2];

fn mod_n_2(v: &mut Vec2, n: f32) {
    v.iter_mut().for_each(|e| {
        *e = *e - (*e / n).floor() * n;
    });
}

fn mod_n_3(v: &mut Vec3, n: f32) {
    v.iter_mut().for_each(|e| {
        *e = *e - (*e / n).floor() * n;
    });
}

fn permute_3(v: &Vec3, n: f32, m: f32, b: f32) -> Vec3 {
    let mut result = times_3(&[v[0] * m + b, v[1] * m + b, v[2] * m + b], v);
    mod_n_3(&mut result, 289.);
    result
}

fn floor_2(v: &Vec2) -> Vec2 {
    [v[0].floor(), v[1].floor()]
}

fn floor_3(v: &Vec3) -> Vec3 {
    [v[0].floor(), v[1].floor(), v[2].floor()]
}

fn plus_2(v1: &Vec2, v2: &Vec2) -> Vec2 {
    [v1[0] + v2[0], v1[1] + v2[1]]
}

fn minus_2(v1: &Vec2, v2: &Vec2) -> Vec2 {
    [v1[0] - v2[0], v1[1] - v2[1]]
}

fn plus_3(v1: &Vec3, v2: &Vec3) -> Vec3 {
    [v1[0] + v2[0], v1[1] + v2[1], v1[2] + v2[2]]
}

fn minus_3(v1: &Vec3, v2: &Vec3) -> Vec3 {
    [v1[0] - v2[0], v1[1] - v2[1], v1[2] - v2[2]]
}

fn times_3(v1: &Vec3, v2: &Vec3) -> Vec3 {
    [v1[0] * v2[0], v1[1] * v2[1], v1[2] * v2[2]]
}

fn dot_2(v1: &Vec2, v2: &Vec2) -> f32 {
    v1[0] * v2[0] + v1[1] * v2[1]
}

fn dot_3(v1: &Vec3, v2: &Vec3) -> f32 {
    v1[0] * v2[0] + v1[1] * v2[1] + v1[2] * v2[2]
}

fn vec_2(v: f32) -> Vec2 {
    [v, v]
}

fn vec_3(v: f32) -> Vec3 {
    [v, v, v]
}

pub struct SimplexGenerator {}

impl SimplexGenerator {
    pub fn generate(&self, point: Point) -> f32 {
        let v: Vec2 = [point.x as f32, point.y as f32];
        let mut i = floor_2(&plus_2(&v, &vec_2(dot_2(&v, &[CY, CY]))));
        let x0 = plus_2(&minus_2(&v, &i), &vec_2(dot_2(&i, &[CX, CX])));

        let bottom_half = x0[0] > x0[1];
        let i1 = [bottom_half as i32 as f32, (!bottom_half) as i32 as f32];

        let x12_top = minus_2(&plus_2(&x0, &[CX, CX]), &i1);
        let x12_bottom = plus_2(&x0, &[CZ, CZ]);

        mod_n_2(&mut i, 289.0);

        let mut p = permute_3(&[i[1], i1[1] + i[1], i[1] + 1.], 289., 34., 1.);

        p = permute_3(
            &plus_3(&p, &[i[0], i[0] + i1[0], i[0] + 1.]),
            289.,
            34.,
            1.,
        );

        let mut m = [
            0.0_f32.max(0.5 - dot_2(&x0, &x0)),
            0.0_f32.max(0.5 - dot_2(&x12_top, &x12_top)),
            0.0_f32.max(0.5 - dot_2(&x12_bottom, &x12_bottom)),
        ];

        m = times_3(&m, &m);
        m = times_3(&m, &m);

        let x = [
            2.0f32 * (p[0] * CW).fract() - 1.0,
            2.0f32 * (p[1] * CW).fract() - 1.0,
            2.0f32 * (p[2] * CW).fract() - 1.0,
        ];

        let h = [x[0].abs() - 0.5, x[1].abs() - 0.5, x[2].abs() - 0.5];

        let ox = floor_3(&plus_3(&x, &vec_3(0.5)));
        let a0 = minus_3(&x, &ox);

        m = times_3(
            &m,
            &minus_3(
                &vec_3(1.7928429),
                &times_3(
                    &vec_3(0.85373473),
                    &times_3(&times_3(&a0, &a0), &times_3(&h, &h)),
                ),
            ),
        );

        let g = [
            a0[0] * x0[0] + h[0] * x0[1],
            a0[1] * x12_top[0] + h[1] * x12_top[1],
            a0[2] * x12_bottom[0] + h[2] * x12_bottom[1],
        ];

        130.0_f32 * dot_3(&m, &g)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_generate() {
        let g = SimplexGenerator {};
        let v = g.generate(Point::new(1., 1.));
    }

    #[test]
    fn test_x0_variability() {
        let v: Vec2 = [101003.0, 230234.0];
        let mut i = floor_2(&plus_2(&v, &vec_2(dot_2(&v, &[CY, CY]))));
        let x0_a = plus_2(&minus_2(&v, &i), &vec_2(dot_2(&i, &[CX, CX])));

        let v: Vec2 = [1., 1.];
        let mut i = floor_2(&plus_2(&v, &vec_2(dot_2(&v, &[CY, CY]))));
        let x0_b = plus_2(&minus_2(&v, &i), &vec_2(dot_2(&i, &[CX, CX])));

        let diff = minus_2(&x0_a, &x0_b);
    }
}
