use super::SpriteSource;

use crate::ui::HasMutableLocation;

pub trait Viewport:
    'static + Send + Sync + SpriteSource + HasMutableLocation
{
}
