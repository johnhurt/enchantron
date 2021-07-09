use std::ops::{Add, AddAssign, Mul, MulAssign, Sub};

use super::{IPoint, Size};

#[derive(Default, Debug, Clone, PartialEq, Copy)]
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
        Point { x, y }
    }

    pub fn distance_to(&self, point: &Point) -> f64 {
        self.distance_squared_to(point).sqrt()
    }

    pub fn distance_squared_to(&self, point: &Point) -> f64 {
        let dx = self.x - point.x;
        let dy = self.y - point.y;

        dx * dx + dy * dy
    }

    /// Perform a dot product operation between this point and the other
    pub fn dot(&self, other: &Point) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn floor(&self) -> IPoint {
        IPoint {
            x: self.x.floor() as i64,
            y: self.y.floor() as i64,
        }
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
        *self * rhs
    }
}

impl MulAssign<f64> for Point {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<'a, 'b> Sub<&'a Point> for &'b Point {
    type Output = Point;

    fn sub(self, rhs: &'a Point) -> Point {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Add<Point> for &Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Add<&Point> for Point {
    type Output = Point;

    fn add(mut self, rhs: &Point) -> Point {
        self += rhs;
        self
    }
}

impl<'a> AddAssign<&'a Point> for Point {
    fn add_assign(&mut self, rhs: &'a Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(mut self, rhs: Point) -> Point {
        self += &rhs;
        self
    }
}

impl<'a> Add<&'a Size> for &Point {
    type Output = Point;

    fn add(self, rhs: &'a Size) -> Point {
        Point::new(self.x + rhs.width, self.y + rhs.height)
    }
}

impl Add<Size> for Point {
    type Output = Point;

    fn add(self, rhs: Size) -> Point {
        Point::new(self.x + rhs.width, self.y + rhs.height)
    }
}

#[test]
fn test_distance_to() {
    let p1 = Point::new(0., 0.);
    let p2 = Point::new(3., 4.);

    assert_eq!(p1.distance_to(&p2), 5.);
}
