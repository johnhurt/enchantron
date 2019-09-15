use super::SpriteSource;

use crate::ui::{HasMutableLocation, HasMutableScale};

pub trait Viewport:
    'static + Send + Sync + SpriteSource + HasMutableLocation + HasMutableScale
{
}
