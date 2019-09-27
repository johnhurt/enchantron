use super::{IPoint, ISize};

#[derive(Default, Debug, Clone)]
pub struct IRect {
    pub top_left: IPoint,
    pub i_size: ISize,
}

impl IRect {
    pub fn new(left: i64, top: i64, width: usize, height: usize) -> IRect {
        IRect {
            top_left: IPoint::new(left, top),
            i_size: ISize::new(width, height),
        }
    }

    pub fn center(&self) -> IPoint {
        IPoint {
            x: self.top_left.x + self.i_size.width as i64 / 2,
            y: self.top_left.y + self.i_size.height as i64 / 2,
        }
    }

    /// Get the minimum distance from this IRect to the given IPoint.  If the given
    /// IPoint is within this IRectangle then 0 is returned
    pub fn distance_to(&self, i_point: &IPoint) -> f64 {
        if self.contains(i_point) {
            return 0.;
        }

        let right = self.top_left.x + self.i_size.width as i64;
        let bottom = self.top_left.y + self.i_size.height as i64;

        if i_point.x < self.top_left.x {
            if i_point.y < self.top_left.y {
                i_point.distance_to(&self.top_left)
            } else if i_point.y > bottom {
                i_point.distance_to(&IPoint::new(self.top_left.x, bottom))
            } else {
                (self.top_left.x - i_point.x) as f64
            }
        } else if i_point.x > right {
            if i_point.y < self.top_left.y {
                i_point.distance_to(&IPoint::new(right, self.top_left.y))
            } else if i_point.y > bottom {
                i_point.distance_to(&IPoint::new(right, bottom))
            } else {
                (i_point.x - right) as f64
            }
        } else {
            if i_point.y < self.top_left.y {
                (self.top_left.y - i_point.y) as f64
            } else {
                (i_point.y - bottom) as f64
            }
        }
    }

    /// Return whether or not the given IPoint is within the given IRectangle
    pub fn contains(&self, i_point: &IPoint) -> bool {
        !(i_point.x < self.top_left.x
            || i_point.x > self.top_left.x + self.i_size.width as i64
            || i_point.y < self.top_left.y
            || i_point.y > self.top_left.y + self.i_size.height as i64)
    }
}

#[test]
fn test_contains() {
    let r = IRect::new(-1., -2., 3., 4.);

    assert_eq!(r.contains(&IPoint::new(0., 0.)), true);
    assert_eq!(r.contains(&IPoint::new(-1.000001, 0.)), false);
}

#[test]
fn test_distance() {
    let r = IRect::new(-1., -2., 3., 4.);

    assert_eq!(r.distance_to(&IPoint::new(0., 0.)), 0.); // in
    assert_eq!(r.distance_to(&IPoint::new(0., 3.)), 1.); // above
    assert_eq!(r.distance_to(&IPoint::new(0., -4.)), 2.); // under
    assert_eq!(r.distance_to(&IPoint::new(-4., 0.)), 3.); // left
    assert_eq!(r.distance_to(&IPoint::new(6., 0.)), 4.); //right

    assert_eq!(r.distance_to(&IPoint::new(-5., -5.)), 5.); // under left
    assert_eq!(r.distance_to(&IPoint::new(-4., -6.)), 5.); // under left
    assert_eq!(r.distance_to(&IPoint::new(5., -6.)), 5.); // under right
    assert_eq!(r.distance_to(&IPoint::new(6., -5.)), 5.); // under right
    assert_eq!(r.distance_to(&IPoint::new(-4., 6.)), 5.); // over left
    assert_eq!(r.distance_to(&IPoint::new(-5., 5.)), 5.); // over left
    assert_eq!(r.distance_to(&IPoint::new(5., 6.)), 5.); // over right
    assert_eq!(r.distance_to(&IPoint::new(6., 5.)), 5.); // over right
}
