use super::{ISize, Point};
use rstar::Point as RTreePoint;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
        IPoint { x, y }
    }

    /// Get the distance to the given point
    pub fn distance_to(&self, i_point: &IPoint) -> f64 {
        (self.distance_squared(i_point) as f64).sqrt()
    }

    /// Get the distance squared from this point to the given point
    pub fn distance_squared(&self, i_point: &IPoint) -> i64 {
        let dx = self.x - i_point.x;
        let dy = self.y - i_point.y;

        dx * dx + dy * dy
    }

    pub fn to_size(&self) -> Option<ISize> {
        if self.x >= 0 && self.y >= 0 {
            Some(ISize::new(self.x as usize, self.y as usize))
        } else {
            None
        }
    }

    pub fn length_2(&self) -> i64 {
        self.x * self.x + self.y + self.y
    }

    /// Create a new point that is the component-wise maximum of this point and
    /// the given point
    pub fn component_max(&self, other: &IPoint) -> IPoint {
        IPoint::new(self.x.max(other.x), self.y.max(other.y))
    }

    /// Create a new point that is the component-wise minimum of this point and
    /// the given point
    pub fn component_min(&self, other: &IPoint) -> IPoint {
        IPoint::new(self.x.min(other.x), self.y.min(other.y))
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

impl Div<i64> for IPoint {
    type Output = IPoint;

    fn div(mut self, rhs: i64) -> IPoint {
        self /= rhs;
        self
    }
}

impl Div<i64> for &IPoint {
    type Output = IPoint;

    fn div(self, rhs: i64) -> IPoint {
        self.clone() / rhs
    }
}

impl DivAssign<i64> for IPoint {
    fn div_assign(&mut self, rhs: i64) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl AddAssign<&IPoint> for IPoint {
    fn add_assign(&mut self, rhs: &IPoint) {
        self.x += rhs.x;
        self.y += rhs.y;
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

impl RTreePoint for IPoint {
    type Scalar = i64;
    const DIMENSIONS: usize = 2;

    fn generate(generator: impl Fn(usize) -> Self::Scalar) -> Self {
        IPoint::new(generator(0), generator(1))
    }

    fn nth(&self, index: usize) -> Self::Scalar {
        if index == 0 {
            self.x
        } else {
            self.y
        }
    }

    fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar {
        if index == 0 {
            &mut self.x
        } else {
            &mut self.y
        }
    }
}

#[test]
fn test_distance_to() {
    let p1 = IPoint::new(0, 0);
    let p2 = IPoint::new(3, 4);

    assert_eq!(p1.distance_to(&p2), 5.);
}
