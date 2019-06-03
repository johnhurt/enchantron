use crate::native::Texture;
use crate::ui::{Sprite, SpriteSource};

pub struct GameDisplayState<S>
where
    S: Sprite,
{
    pub grass: S,
}

impl<T, S> GameDisplayState<S>
where
    T: Texture,
    S: Sprite<T = T>,
{
    pub fn new<SS: SpriteSource<T = T, S = S>>(
        sprite_source: &SS,
    ) -> GameDisplayState<S> {
        GameDisplayState {
            grass: sprite_source.create_sprite(),
        }
    }
}
