use super::SpriteSource;

pub trait Viewport: 'static + Send + Sync + SpriteSource {}
