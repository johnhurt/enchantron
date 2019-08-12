use super::{HasMutableLocation, HasMutableSize};

pub trait Viewport: HasMutableLocation + HasMutableSize {}
