use std::ops::{Mul, MulAssign};

#[derive(Default, Debug, Clone)]
pub struct IPoint {
    pub x: usize,
    pub y: usize,
}

impl IPoint {
    /// Create a new IPoint with the given x and y IPoint:
    /// ```
    /// let p = IPoint::new(1.0, -2.0);
    /// assert_eq!(p.x, 1.0);
    /// assert_eq!(p.y, -2.0);
    /// ```
    pub fn new(x: usize, y: usize) -> IPoint {
        IPoint { x: x, y: y }
    }

    pub fn distance_to(&self, IPoint: &IPoint) -> f64 {
        let dx = self.x - IPoint.x;
        let dy = self.y - IPoint.y;

        ((dx * dx + dy * dy) as f64).sqrt()
    }
}

impl Mul<usize> for IPoint {
    type Output = IPoint;

    fn mul(mut self, rhs: usize) -> IPoint {
        self *= rhs;
        self
    }
}

impl Mul<usize> for &IPoint {
    type Output = IPoint;

    fn mul(self, rhs: usize) -> IPoint {
        self.clone() * rhs
    }
}

impl MulAssign<usize> for IPoint {
    fn mul_assign(&mut self, rhs: usize) {
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
