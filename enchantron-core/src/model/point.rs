use std::ops::{Mul, MulAssign};

#[derive(Default, Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    /// Create a new point with the given x and y point:
    /// ```
    /// let p = Point::new(1.0, -2.0);
    /// assert_eq!(p.x, 1.0);
    /// assert_eq!(p.y, -2.0);
    /// ```
    pub fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }

    pub fn distance_to(&self, point: &Point) -> f64 {
        let dx = self.x - point.x;
        let dy = self.y - point.y;

        (dx * dx + dy * dy).sqrt()
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(mut self, rhs: f64) -> Point {
        self *= rhs;
        self
    }
}

impl Mul<f64> for &Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Point {
        self.clone() * rhs
    }
}

impl MulAssign<f64> for Point {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

#[test]
fn test_distance_to() {
    let p1 = Point::new(0., 0.);
    let p2 = Point::new(3., 4.);

    assert_eq!(p1.distance_to(&p2), 5.);
}
