use std::ops::{Mul, MulAssign};

#[derive(Default, Clone, Debug)]
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
        self.clone() * rhs
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

    pub fn aspect_ratio(&self) -> usize {
        self.width / self.height
    }

    pub fn is_zero(&self) -> bool {
        self.width == 0 && self.height == 0
    }
}
