#[derive(Default, Debug, Clone)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

impl Size {
    pub fn new(width: f64, height: f64) -> Size {
        Size {
            width: width,
            height: height,
        }
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.width / self.height
    }
}
