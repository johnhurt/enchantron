use std::ops::{Mul, MulAssign};

#[derive(Default, Debug, Clone)]
pub struct UPoint {
    pub x: usize,
    pub y: usize,
}

impl UPoint {
    /// Create a new UPoint with the given x and y UPoint:
    /// ```
    /// let p = UPoint::new(1.0, -2.0);
    /// assert_eq!(p.x, 1.0);
    /// assert_eq!(p.y, -2.0);
    /// ```
    pub fn new(x: usize, y: usize) -> UPoint {
        UPoint { x: x, y: y }
    }

    pub fn distance_to(&self, i_point: &UPoint) -> f64 {
        let dx = (self.x - i_point.x) as f64;
        let dy = (self.y - i_point.y) as f64;

        (dx * dx + dy * dy).sqrt()
    }
}

impl Mul<usize> for UPoint {
    type Output = UPoint;

    fn mul(mut self, rhs: usize) -> UPoint {
        self *= rhs;
        self
    }
}

impl Mul<usize> for &UPoint {
    type Output = UPoint;

    fn mul(self, rhs: usize) -> UPoint {
        self.clone() * rhs
    }
}

impl MulAssign<usize> for UPoint {
    fn mul_assign(&mut self, rhs: usize) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

#[test]
fn test_distance_to() {
    let p1 = UPoint::new(0., 0.);
    let p2 = UPoint::new(3., 4.);

    assert_eq!(p1.distance_to(&p2), 5.);
}
