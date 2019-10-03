use std::ops::{Add, Mul, MulAssign, Sub};

use super::{ISize, Point};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
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

    pub fn to_size(&self) -> Option<ISize> {
        if self.x >= 0 && self.y >= 0 {
            Some(ISize::new(self.x as usize, self.y as usize))
        } else {
            None
        }
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

impl Mul<f64> for &IPoint {
    type Output = Point;

    fn mul(self, rhs: f64) -> Point {
        Point::new(self.x as f64 * rhs, self.y as f64 * rhs)
    }
}

impl MulAssign<i64> for IPoint {
    fn mul_assign(&mut self, rhs: i64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<'a, 'b> Add<&'b IPoint> for &'a IPoint {
    type Output = IPoint;

    fn add(self, rhs: &'b IPoint) -> IPoint {
        IPoint {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<'a, 'b> Sub<&'b IPoint> for &'a IPoint {
    type Output = IPoint;

    fn sub(self, rhs: &'b IPoint) -> IPoint {
        IPoint {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<'b> Sub<&'b IPoint> for IPoint {
    type Output = IPoint;

    fn sub(mut self, rhs: &'b IPoint) -> IPoint {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self
    }
}

#[test]
fn test_distance_to() {
    let p1 = IPoint::new(0, 0);
    let p2 = IPoint::new(3, 4);

    assert_eq!(p1.distance_to(&p2), 5.);
}
