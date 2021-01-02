use super::NativeView;
use crate::event::RawTouchEvent;
use crate::model::{Rect, Size};
use crate::ui::{Button, ButtonPublic, Color, TouchEventType};
use crate::view_impl;
use crate::view_types::ViewTypes;

const MAX_WIDTH_FRAC: f64 = 0.7;
const HEIGHT_FRAC: f64 = 0.2;
const BUTTON_ASPECT_RATIO: f64 = 1.618;

/// Calculate the rectangle for the loading progress bar based on the size
/// of the screen
fn calculate_rect_from_size(size: Size) -> Rect {
    let max_height = size.height * HEIGHT_FRAC;
    let max_width = size.width * MAX_WIDTH_FRAC;

    let width = max_width.min(max_height * BUTTON_ASPECT_RATIO);
    let height = width / BUTTON_ASPECT_RATIO;

    let x = size.width / 2. - width / 2.;
    let y = size.height / 2. - height / 2.;

    Rect::new(x, y, width, height)
}

pub trait MainMenuView: 'static + Sized + Send + Sync + NativeView {
    type B: Button;

    fn transition_to_game_view(&self);

    fn get_start_new_game_button(&self) -> Self::B;
}

view_impl!(MainMenuView<T> {
    widgets {
        start_new_game_button: Button
    }



    init = init;
    on_layout = on_layout;
    on_touch = on_touch;
});

impl<T> MainMenuView for MainMenuViewPublic<T>
where
    T: ViewTypes<Button = ButtonPublic<T>>,
{
    type B = T::Button;

    fn transition_to_game_view(&self) {}

    fn get_start_new_game_button(&self) -> Self::B {
        self.start_new_game_button.clone()
    }
}

impl<T> MainMenuViewPrivate<T>
where
    T: ViewTypes,
{
    fn init(&mut self) {
        self.start_new_game_button
            .set_color(T::Color::new(123, 190, 200, 255))
    }

    fn on_layout(&mut self, size: Size) {
        let new_game_button_rect = calculate_rect_from_size(size);
        self.start_new_game_button.set_rect(new_game_button_rect);
    }

    fn on_touch(&mut self, touch_event: RawTouchEvent) {
        if touch_event.state == TouchEventType::End
            && self
                .start_new_game_button
                .rect
                .contains_point(&touch_event.touch.point)
        {
            self.start_new_game_button.on_click()
        }
    }
}
