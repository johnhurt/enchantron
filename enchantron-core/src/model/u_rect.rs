use super::{ISize, UPoint};

#[derive(Default, Debug, Clone)]
pub struct URect {
    pub top_left: UPoint,
    pub size: ISize,
}

impl URect {
    pub fn new(left: usize, top: usize, width: usize, height: usize) -> URect {
        URect {
            top_left: UPoint::new(left, top),
            size: ISize::new(width, height),
        }
    }

    pub fn center(&self) -> UPoint {
        UPoint {
            x: self.top_left.x + self.size.width as usize / 2,
            y: self.top_left.y + self.size.height as usize / 2,
        }
    }

    /// Get the minimum distance from this URect to the given UPoint.  If the given
    /// UPoint is within this URectangle then 0 is returned
    pub fn distance_to(&self, u_point: &UPoint) -> f64 {
        if self.contains(u_point) {
            return 0.;
        }

        let right = self.top_left.x + self.size.width as usize;
        let bottom = self.top_left.y + self.size.height as usize;

        if u_point.x < self.top_left.x {
            if u_point.y < self.top_left.y {
                u_point.distance_to(&self.top_left)
            } else if u_point.y > bottom {
                u_point.distance_to(&UPoint::new(self.top_left.x, bottom))
            } else {
                (self.top_left.x - u_point.x) as f64
            }
        } else if u_point.x > right {
            if u_point.y < self.top_left.y {
                u_point.distance_to(&UPoint::new(right, self.top_left.y))
            } else if u_point.y > bottom {
                u_point.distance_to(&UPoint::new(right, bottom))
            } else {
                (u_point.x - right) as f64
            }
        } else {
            if u_point.y < self.top_left.y {
                (self.top_left.y - u_point.y) as f64
            } else {
                (u_point.y - bottom) as f64
            }
        }
    }

    /// Return whether or not the given UPoint is within the given URectangle
    pub fn contains(&self, u_point: &UPoint) -> bool {
        !(u_point.x < self.top_left.x
            || u_point.x > self.top_left.x + self.size.width as usize
            || u_point.y < self.top_left.y
            || u_point.y > self.top_left.y + self.size.height as usize)
    }
}

#[test]
fn test_contains() {
    let r = URect::new(9, 8, 13, 14);

    assert_eq!(r.contains(&UPoint::new(10, 10)), true);
    assert_eq!(r.contains(&UPoint::new(8, 0)), false);
}

#[test]
fn test_distance() {
    let r = URect::new(9, 8, 3, 4);

    assert_eq!(r.distance_to(&UPoint::new(10, 10)), 0.); // in
    assert_eq!(r.distance_to(&UPoint::new(10, 13)), 1.); // above
    assert_eq!(r.distance_to(&UPoint::new(10, 6)), 2.); // under
    assert_eq!(r.distance_to(&UPoint::new(6, 10)), 3.); // left
    assert_eq!(r.distance_to(&UPoint::new(16, 10)), 4.); //right

    assert_eq!(r.distance_to(&UPoint::new(5, 5)), 5.); // under left
    assert_eq!(r.distance_to(&UPoint::new(6, 4)), 5.); // under left
    assert_eq!(r.distance_to(&UPoint::new(15, 4)), 5.); // under right
    assert_eq!(r.distance_to(&UPoint::new(16, 5)), 5.); // under right
    assert_eq!(r.distance_to(&UPoint::new(6, 16)), 5.); // over left
    assert_eq!(r.distance_to(&UPoint::new(5, 15)), 5.); // over left
    assert_eq!(r.distance_to(&UPoint::new(15, 16)), 5.); // over right
    assert_eq!(r.distance_to(&UPoint::new(16, 15)), 5.); // over right
}
