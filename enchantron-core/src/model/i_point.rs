use std::ops::{Mul, MulAssign};

#[derive(Default, Debug, Clone)]
pub struct IPoint {
    pub x: i64,
    pub y: i64,
}

impl IPoint {
    /// Create a new IPoint with the given x and y IPoint:
    /// ```
    /// let p = IPoint::new(1.0, -2.0);
    /// assert_eq!(p.x, 1.0);
    /// assert_eq!(p.y, -2.0);
    /// ```
    pub fn new(x: i64, y: i64) -> IPoint {
        IPoint { x: x, y: y }
    }

    pub fn distance_to(&self, i_point: &IPoint) -> f64 {
        let dx = (self.x - i_point.x) as f64;
        let dy = (self.y - i_point.y) as f64;

        (dx * dx + dy * dy).sqrt()
    }
}

impl Mul<i64> for IPoint {
    type Output = IPoint;

    fn mul(mut self, rhs: i64) -> IPoint {
        self *= rhs;
        self
    }
}

impl Mul<i64> for &IPoint {
    type Output = IPoint;

    fn mul(self, rhs: i64) -> IPoint {
        self.clone() * rhs
    }
}

impl MulAssign<i64> for IPoint {
    fn mul_assign(&mut self, rhs: i64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

#[test]
fn test_distance_to() {
    let p1 = IPoint::new(0., 0.);
    let p2 = IPoint::new(3., 4.);

    assert_eq!(p1.distance_to(&p2), 5.);
}
