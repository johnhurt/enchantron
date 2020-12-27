use super::{
    HasMutableColor, HasMutableLocation, HasMutableSize, HasMutableVisibility,
    Sprite, SpriteGroup, SpriteSource,
};
use crate::model::Rect;
use crate::view_types::ViewTypes;

#[derive(Debug)]
pub struct ProgressBar<T: ViewTypes> {
    outline: T::Sprite,
    bar: T::Sprite,

    rect: Rect,
    value: f64,
}

impl<T> ProgressBar<T>
where
    T: ViewTypes,
{
    pub fn new(
        sprite_source: &impl SpriteSource<T = T::Texture, S = T::Sprite>,
    ) -> ProgressBar<T> {
        let outline = sprite_source.create_sprite();
        let bar = sprite_source.create_sprite();

        ProgressBar {
            outline,
            bar,
            rect: Rect::default(),
            value: 0.,
        }
    }
}
