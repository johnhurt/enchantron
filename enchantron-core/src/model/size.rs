use std::ops::{Mul, MulAssign};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

impl Mul<f64> for Size {
    type Output = Size;

    fn mul(mut self, rhs: f64) -> Size {
        self *= rhs;
        self
    }
}

impl Mul<f64> for &Size {
    type Output = Size;

    fn mul(self, rhs: f64) -> Size {
        self.clone() * rhs
    }
}

impl MulAssign<f64> for Size {
    fn mul_assign(&mut self, rhs: f64) {
        self.width *= rhs;
        self.height *= rhs;
    }
}

impl Size {
    pub fn new(width: f64, height: f64) -> Size {
        Size { width, height }
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.width / self.height
    }

    pub fn is_zero(&self) -> bool {
        self.width == 0. && self.height == 0.
    }
}
