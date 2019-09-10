use crate::model::Rect;
use crate::native::Texture;
use crate::ui::{DragState, Sprite, SpriteSource};

pub struct GameDisplayState<S>
where
    S: Sprite,
{
    pub grass: S,
    pub viewport_rect: Rect,
    pub drag_state: Option<DragState>,
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
            drag_state: Default::default(),
            viewport_rect: Default::default(),
        }
    }
}
