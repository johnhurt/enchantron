use super::{IPoint, ISize};
use rstar::{Envelope, Point, PointDistance, RTreeObject};
use std::cmp::{max, min};

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
pub struct IRect {
    pub top_left: IPoint,
    pub size: ISize,
}

#[allow(dead_code)]
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

    /// Get the last included point in this rect. This will be the coordinates
    /// of the ipoint at the bottom right of the rectangle that is within the
    /// integer size of the rect. For rectangles with zero size, the top-left
    /// point is returned even though that isn't technically correct, but a rect
    /// with zero size isn't technically a rect, so it's kind of on you
    pub fn bottom_right_inclusive(&self) -> IPoint {
        let dx = max(1, self.size.width as i64) - 1;
        let dy = max(1, self.size.height as i64) - 1;
        IPoint {
            x: self.top_left.x + dx,
            y: self.top_left.y + dy,
        }
    }

    /// Get the coordinates of the point that touches the bottom right corner
    /// of this rect, but is not included in the rect un less the rect has
    /// zero size
    pub fn bottom_right_exclusive(&self) -> IPoint {
        IPoint {
            x: self.top_left.x + self.size.width as i64,
            y: self.top_left.y + self.size.height as i64,
        }
    }

    /// Get the distance squared to the given point
    pub fn distance_squared(&self, i_point: &IPoint) -> i64 {
        if self.contains_point(i_point) {
            return 0;
        }

        let right = self.top_left.x + self.size.width as i64;
        let bottom = self.top_left.y + self.size.height as i64;

        if i_point.x < self.top_left.x {
            if i_point.y < self.top_left.y {
                i_point.distance_2(&self.top_left)
            } else if i_point.y > bottom {
                i_point.distance_2(&IPoint::new(self.top_left.x, bottom))
            } else {
                let x = self.top_left.x - i_point.x;
                x * x
            }
        } else if i_point.x > right {
            if i_point.y < self.top_left.y {
                i_point.distance_2(&IPoint::new(right, self.top_left.y))
            } else if i_point.y > bottom {
                i_point.distance_2(&IPoint::new(right, bottom))
            } else {
                let x = i_point.x - right;
                x * x
            }
        } else {
            let d = if i_point.y < self.top_left.y {
                self.top_left.y - i_point.y
            } else {
                i_point.y - bottom
            };

            d * d
        }
    }

    /// Get the minimum distance from this IRect to the given IPoint.  If the
    /// given IPoint is within this IRectangle then 0 is returned
    pub fn distance_to(&self, i_point: &IPoint) -> f64 {
        (self.distance_squared(i_point) as f64).sqrt()
    }

    /// Return whether or not the given IPoint is within the given IRectangle
    pub fn contains_point(&self, i_point: &IPoint) -> bool {
        let dx = max(self.size.width as i64, 1);
        let dy = max(self.size.height as i64, 1);

        !(i_point.x < self.top_left.x
            || i_point.x >= self.top_left.x + dx
            || i_point.y < self.top_left.y
            || i_point.y >= self.top_left.y + dy)
    }

    /// perform the given action on all 4 corners of this rect starting at the
    /// top left and going around clockwise (top-left, top-right, bottom-right,
    /// bottom-left), and returns a 4tuple with the result of the action at each
    /// corner in the same order
    pub fn for_each_corner<T>(
        &self,
        action: impl Fn(&IPoint) -> T,
    ) -> (T, T, T, T) {
        let mut corner = self.top_left;

        // top left
        let t1 = action(&corner);

        // top right
        corner.x += self.size.width as i64;
        let t2 = action(&corner);

        // bottom right
        corner.y += self.size.height as i64;
        let t3 = action(&corner);

        // bottom left
        corner.x = self.top_left.x;
        let t4 = action(&corner);

        (t1, t2, t3, t4)
    }

    pub fn area(&self) -> usize {
        self.size.area()
    }

    /// Returns whether or not the given rect is completely within the bounds
    /// of this rect
    pub fn contains_rect(&self, rect: &IRect) -> bool {
        self.contains_point(&rect.top_left)
            && self.contains_point(&rect.bottom_right_inclusive())
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

        let signed_size = &new_bottom_right - &new_top_left;

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

    /// Create a new rect by expanding this rect in all directions by the
    /// given offset
    pub fn expanded_by(&self, offset: usize) -> IRect {
        IRect {
            top_left: IPoint {
                x: self.top_left.x - offset as i64,
                y: self.top_left.y - offset as i64,
            },
            size: ISize {
                width: self.size.width + 2 * offset,
                height: self.size.height + 2 * offset,
            },
        }
    }
}

impl Envelope for IRect {
    type Point = IPoint;

    fn new_empty() -> Self {
        IRect::default()
    }

    fn contains_point(&self, point: &IPoint) -> bool {
        self.contains_point(point)
    }

    fn contains_envelope(&self, other: &IRect) -> bool {
        self.contains_rect(other)
    }

    fn merge(&mut self, other: &Self) {
        let bottom_right = self
            .bottom_right_exclusive()
            .component_max(&other.bottom_right_exclusive());
        self.top_left = self.top_left.component_min(&other.top_left);
        self.size = (bottom_right - &self.top_left).to_size().unwrap()
    }

    fn merged(&self, other: &Self) -> Self {
        let mut result = self.clone();
        result.merge(other);
        result
    }

    fn intersects(&self, other: &Self) -> bool {
        self.intersection(other).is_some()
    }

    fn area(&self) -> i64 {
        self.area() as i64
    }

    fn distance_2(&self, point: &Self::Point) -> i64 {
        self.distance_squared(point)
    }

    fn min_max_dist_2(&self, point: &Self::Point) -> i64 {
        let l_x = self.top_left.x - point.x;
        let l_y = self.top_left.y - point.y;
        let u_x = l_x + self.size.width as i64;
        let u_y = l_y + self.size.height as i64;

        let (min_x, max_x) = if l_x.abs() < u_x.abs() {
            (l_x, u_x)
        } else {
            (u_x, l_x)
        };

        let (min_y, max_y) = if l_y.abs() < u_y.abs() {
            (l_y, u_y)
        } else {
            (u_y, l_y)
        };

        let l_sq_1 = min_x * min_x + max_y * max_y;
        let l_sq_2 = max_x * max_x + min_y * min_y;

        l_sq_1.min(l_sq_2)
    }

    fn center(&self) -> Self::Point {
        IPoint::new(
            self.top_left.x + (self.size.width / 2) as i64,
            self.top_left.y + (self.size.height / 2) as i64,
        )
    }

    fn intersection_area(
        &self,
        other: &Self,
    ) -> <Self::Point as Point>::Scalar {
        self.intersection(other)
            .map(|intersection| intersection.area())
            .unwrap_or_default() as i64
    }

    fn perimeter_value(&self) -> <Self::Point as Point>::Scalar {
        (self.size.width + self.size.height) as i64
    }

    fn sort_envelopes<T: RTreeObject<Envelope = Self>>(
        axis: usize,
        envelopes: &mut [T],
    ) {
        envelopes.sort_by(|l, r| {
            let ref lp = l.envelope().top_left;
            let ref rp = r.envelope().top_left;

            if axis == 0 {
                lp.x.cmp(&rp.x)
            } else {
                lp.y.cmp(&rp.y)
            }
        });
    }

    fn partition_envelopes<T: RTreeObject<Envelope = Self>>(
        axis: usize,
        envelopes: &mut [T],
        selection_size: usize,
    ) {
        pdqselect::select_by(envelopes, selection_size, |l, r| {
            let ref lp = l.envelope().top_left;
            let ref rp = r.envelope().top_left;

            if axis == 0 {
                lp.x.cmp(&rp.x)
            } else {
                lp.y.cmp(&rp.y)
            }
        });
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
        size: ISize {
            width: 4,
            height: 4,
        },
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

#[test]
fn test_intersection() {
    let mut rect = IRect::default();

    assert_eq!(rect.intersection(&rect).as_ref(), None);

    rect.size = ISize::new(1, 1);

    assert_eq!(rect.intersection(&rect).as_ref(), Some(&rect));

    let rect1 = IRect::new(0, 0, 4, 4);
    let rect2 = IRect::new(-1, -2, 4, 4);
    let itx1 = rect1.intersection(&rect2);
    let itx2 = rect2.intersection(&rect1);

    assert_eq!(Some(IRect::new(0, 0, 3, 2)), itx1);
    assert_eq!(itx1, itx2);
}
