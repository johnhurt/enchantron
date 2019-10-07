use super::{IPoint, ISize};

use std::cmp::{max, min};

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct IRect {
    pub top_left: IPoint,
    pub size: ISize,
}

impl IRect {
    pub fn new(left: i64, top: i64, width: usize, height: usize) -> IRect {
        IRect {
            top_left: IPoint::new(left, top),
            size: ISize::new(width, height),
        }
    }

    pub fn center(&self) -> IPoint {
        IPoint {
            x: self.top_left.x + self.size.width as i64 / 2,
            y: self.top_left.y + self.size.height as i64 / 2,
        }
    }

    pub fn bottom_right(&self) -> IPoint {
        IPoint {
            x: self.top_left.x + self.size.width as i64,
            y: self.top_left.y + self.size.height as i64,
        }
    }

    /// Get the minimum distance from this IRect to the given IPoint.  If the given
    /// IPoint is within this IRectangle then 0 is returned
    pub fn distance_to(&self, i_point: &IPoint) -> f64 {
        if self.contains_point(i_point) {
            return 0.;
        }

        let right = self.top_left.x + self.size.width as i64;
        let bottom = self.top_left.y + self.size.height as i64;

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
    pub fn contains_point(&self, i_point: &IPoint) -> bool {
        !(i_point.x < self.top_left.x
            || i_point.x > self.top_left.x + self.size.width as i64
            || i_point.y < self.top_left.y
            || i_point.y > self.top_left.y + self.size.height as i64)
    }

    /// Returns whether or not the given rect is completely within the bounds
    /// of this rect
    pub fn contains_rect(&self, rect: &IRect) -> bool {
        self.contains_point(&rect.top_left)
            && self.contains_point(&rect.bottom_right())
    }

    /// Get the intersection between two irects
    pub fn intersection(&self, other: &IRect) -> Option<IRect> {
        let new_top_left = IPoint::new(
            max(self.top_left.x, other.top_left.x),
            max(self.top_left.y, other.top_left.y),
        );

        let new_bottom_right = IPoint::new(
            min(
                self.top_left.x + self.size.width as i64,
                other.top_left.x + other.size.width as i64,
            ),
            min(
                self.top_left.y + self.size.height as i64,
                other.top_left.y + other.size.height as i64,
            ),
        );

        let signed_size = &new_top_left - &new_bottom_right;

        // If both parts of the signed size are positive, there is a non-trivial
        // intersection, so return it
        if signed_size.x > 0 && signed_size.y > 0 {
            Some(IRect {
                top_left: new_top_left,
                size: ISize::new(
                    signed_size.x as usize,
                    signed_size.y as usize,
                ),
            })
        } else {
            None
        }
    }
}

#[test]
fn test_contains_point() {
    let r = IRect::new(-1, -2, 3, 4);

    assert_eq!(r.contains_point(&IPoint::new(0, 0)), true);
    assert_eq!(r.contains_point(&IPoint::new(-2, 0)), false);
}

#[test]
fn test_contains_rect() {
    let r = IRect {
        top_left: IPoint { x: -2, y: -2 },
        size: ISize { width: 4, height: 4 }
    };

    assert!(r.contains_rect(&r));
}

#[test]
fn test_distance() {
    let r = IRect::new(-1, -2, 3, 4);

    assert_eq!(r.distance_to(&IPoint::new(0, 0)), 0.); // in
    assert_eq!(r.distance_to(&IPoint::new(0, 3)), 1.); // above
    assert_eq!(r.distance_to(&IPoint::new(0, -4)), 2.); // under
    assert_eq!(r.distance_to(&IPoint::new(-4, 0)), 3.); // left
    assert_eq!(r.distance_to(&IPoint::new(6, 0)), 4.); //right

    assert_eq!(r.distance_to(&IPoint::new(-5, -5)), 5.); // under left
    assert_eq!(r.distance_to(&IPoint::new(-4, -6)), 5.); // under left
    assert_eq!(r.distance_to(&IPoint::new(5, -6)), 5.); // under right
    assert_eq!(r.distance_to(&IPoint::new(6, -5)), 5.); // under right
    assert_eq!(r.distance_to(&IPoint::new(-4, 6)), 5.); // over left
    assert_eq!(r.distance_to(&IPoint::new(-5, 5)), 5.); // over left
    assert_eq!(r.distance_to(&IPoint::new(5, 6)), 5.); // over right
    assert_eq!(r.distance_to(&IPoint::new(6, 5)), 5.); // over right
}
