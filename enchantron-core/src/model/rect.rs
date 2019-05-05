use super::{Point, Size};

#[derive(Default, Debug, Clone)]
pub struct Rect {
    pub top_left: Point,
    pub size: Size,
}

impl Rect {
    pub fn new(left: f64, top: f64, width: f64, height: f64) -> Rect {
        Rect {
            top_left: Point::new(left, top),
            size: Size::new(width, height),
        }
    }

    pub fn center(&self) -> Point {
        Point {
            x: self.top_left.x + self.size.width / 2.,
            y: self.top_left.y + self.size.height / 2.,
        }
    }

    /// Get the minimum distance from this rect to the given point.  If the given
    /// point is within this rectangle then 0 is returned
    pub fn distance_to(&self, point: &Point) -> f64 {
        if self.contains(point) {
            return 0.;
        }

        let right = self.top_left.x + self.size.width;
        let bottom = self.top_left.y + self.size.height;

        if point.x < self.top_left.x {
            if point.y < self.top_left.y {
                point.distance_to(&self.top_left)
            } else if point.y > bottom {
                point.distance_to(&Point::new(self.top_left.x, bottom))
            } else {
                self.top_left.x - point.x
            }
        } else if point.x > right {
            if point.y < self.top_left.y {
                point.distance_to(&Point::new(right, self.top_left.y))
            } else if point.y > bottom {
                point.distance_to(&Point::new(right, bottom))
            } else {
                point.x - right
            }
        } else {
            if point.y < self.top_left.y {
                self.top_left.y - point.y
            } else {
                point.y - bottom
            }
        }
    }

    /// Return whether or not the given point is within the given rectangle
    pub fn contains(&self, point: &Point) -> bool {
        !(point.x < self.top_left.x
            || point.x > self.top_left.x + self.size.width
            || point.y < self.top_left.y
            || point.y > self.top_left.y + self.size.height)
    }
}

#[test]
fn test_contains() {
    let r = Rect::new(-1., -2., 3., 4.);

    assert_eq!(r.contains(&Point::new(0., 0.)), true);
    assert_eq!(r.contains(&Point::new(-1.000001, 0.)), false);
}

#[test]
fn test_distance() {
    let r = Rect::new(-1., -2., 3., 4.);

    assert_eq!(r.distance_to(&Point::new(0., 0.)), 0.); // in
    assert_eq!(r.distance_to(&Point::new(0., 3.)), 1.); // above
    assert_eq!(r.distance_to(&Point::new(0., -4.)), 2.); // under
    assert_eq!(r.distance_to(&Point::new(-4., 0.)), 3.); // left
    assert_eq!(r.distance_to(&Point::new(6., 0.)), 4.); //right

    assert_eq!(r.distance_to(&Point::new(-5., -5.)), 5.); // under left
    assert_eq!(r.distance_to(&Point::new(-4., -6.)), 5.); // under left
    assert_eq!(r.distance_to(&Point::new(5., -6.)), 5.); // under right
    assert_eq!(r.distance_to(&Point::new(6., -5.)), 5.); // under right
    assert_eq!(r.distance_to(&Point::new(-4., 6.)), 5.); // over left
    assert_eq!(r.distance_to(&Point::new(-5., 5.)), 5.); // over left
    assert_eq!(r.distance_to(&Point::new(5., 6.)), 5.); // over right
    assert_eq!(r.distance_to(&Point::new(6., 5.)), 5.); // over right
}
