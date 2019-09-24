use super::{IPoint, ISize};

#[derive(Default, Debug, Clone)]
pub struct IRect {
    pub top_left: IPoint,
    pub ISize: ISize,
}

impl IRect {
    pub fn new(left: usize, top: usize, width: usize, height: usize) -> IRect {
        IRect {
            top_left: IPoint::new(left, top),
            ISize: ISize::new(width, height),
        }
    }

    pub fn center(&self) -> IPoint {
        IPoint {
            x: self.top_left.x + self.ISize.width / 2,
            y: self.top_left.y + self.ISize.height / 2,
        }
    }

    /// Get the minimum distance from this IRect to the given IPoint.  If the given
    /// IPoint is within this IRectangle then 0 is returned
    pub fn distance_to(&self, IPoint: &IPoint) -> f64 {
        if self.contains(IPoint) {
            return 0.;
        }

        let right = self.top_left.x + self.ISize.width;
        let bottom = self.top_left.y + self.ISize.height;

        if IPoint.x < self.top_left.x {
            if IPoint.y < self.top_left.y {
                IPoint.distance_to(&self.top_left)
            } else if IPoint.y > bottom {
                IPoint.distance_to(&IPoint::new(self.top_left.x, bottom))
            } else {
                (self.top_left.x - IPoint.x) as f64
            }
        } else if IPoint.x > right {
            if IPoint.y < self.top_left.y {
                IPoint.distance_to(&IPoint::new(right, self.top_left.y))
            } else if IPoint.y > bottom {
                IPoint.distance_to(&IPoint::new(right, bottom))
            } else {
                (IPoint.x - right) as f64
            }
        } else {
            if IPoint.y < self.top_left.y {
                (self.top_left.y - IPoint.y) as f64
            } else {
                (IPoint.y - bottom) as f64
            }
        }
    }

    /// Return whether or not the given IPoint is within the given IRectangle
    pub fn contains(&self, IPoint: &IPoint) -> bool {
        !(IPoint.x < self.top_left.x
            || IPoint.x > self.top_left.x + self.ISize.width
            || IPoint.y < self.top_left.y
            || IPoint.y > self.top_left.y + self.ISize.height)
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
