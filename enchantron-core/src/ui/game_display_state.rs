use crate::model::{Point, Rect, Size};
use crate::native::Texture;
use crate::ui::{DragState, Sprite, SpriteSource};

pub struct GameDisplayState<S>
where
    S: Sprite,
{
    pub grass: S,
    pub viewport_rect: Option<Rect>,
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

    /// Update the layout of the display based on a change in the size of
    /// screen
    pub fn layout(&mut self, new_size: Size) {
        if let Some(ref mut viewport_rect) = self.viewport_rect {
            let position_shift = Point::new(
                (viewport_rect.size.width - new_size.width) / 2.,
                (viewport_rect.size.height - new_size.height) / 2.,
            );
            let new_position = Point::new(
                viewport_rect.top_left.x + position_shift.x,
                viewport_rect.top_left.y + position_shift.y,
            );

            viewport_rect.size = new_size;
            viewport_rect.top_left = new_position;
        } else {
            let mut viewport_rect = Rect::default();
            viewport_rect.size = new_size;
            self.viewport_rect = Some(viewport_rect);
        }
    }
}
