use std::ops::{Div, DivAssign, Mul, MulAssign};

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ISize {
    pub width: usize,
    pub height: usize,
}

impl Mul<usize> for ISize {
    type Output = ISize;

    fn mul(mut self, rhs: usize) -> ISize {
        self *= rhs;
        self
    }
}

impl Mul<usize> for &ISize {
    type Output = ISize;

    fn mul(self, rhs: usize) -> ISize {
        *self * rhs
    }
}

impl Div<usize> for ISize {
    type Output = ISize;

    fn div(mut self, rhs: usize) -> ISize {
        self /= rhs;
        self
    }
}

impl Div<usize> for &ISize {
    type Output = ISize;

    fn div(self, rhs: usize) -> ISize {
        self.clone() / rhs
    }
}

impl DivAssign<usize> for ISize {
    fn div_assign(&mut self, rhs: usize) {
        self.width /= rhs;
        self.height /= rhs;
    }
}

impl MulAssign<usize> for ISize {
    fn mul_assign(&mut self, rhs: usize) {
        self.width *= rhs;
        self.height *= rhs;
    }
}

impl ISize {
    pub fn new(width: usize, height: usize) -> ISize {
        ISize {
            width: width,
            height: height,
        }
    }

    pub fn area(&self) -> usize {
        self.height * self.width
    }

    pub fn aspect_ratio(&self) -> usize {
        self.width / self.height
    }

    pub fn is_zero(&self) -> bool {
        self.width == 0 && self.height == 0
    }
}
