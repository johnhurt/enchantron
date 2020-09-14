use std::ops::{Mul, MulAssign};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct UPoint {
    pub x: usize,
    pub y: usize,
}

impl UPoint {
    /// Create a new UPoint with the given x and y UPoint:
    /// ```
    /// let p = UPoint::new(11, 8);
    /// assert_eq!(p.x, 11);
    /// assert_eq!(p.y, 8);
    /// ```
    pub fn new(x: usize, y: usize) -> UPoint {
        UPoint { x, y }
    }

    pub fn distance_to(&self, i_point: &UPoint) -> f64 {
        let dx = if self.x >= i_point.x {
            self.x - i_point.x
        } else {
            i_point.x - self.x
        };
        let dy = if self.y >= i_point.y {
            self.y - i_point.y
        } else {
            i_point.y - self.y
        };

        ((dx * dx + dy * dy) as f64).sqrt()
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
    let p1 = UPoint::new(0, 0);
    let p2 = UPoint::new(3, 4);

    assert_eq!(p1.distance_to(&p2), 5.);
}
