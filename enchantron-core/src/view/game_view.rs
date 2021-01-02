use super::NativeView;
use crate::ui::Button;
use crate::view_impl;
use crate::view_types::ViewTypes;

pub trait GameView: NativeView + Sync + Send + 'static {
    type B: Button;
}

view_impl!(GameView<T> {
    widgets {
        pause_button: Button
    }

    private {

    }
});

impl<T> GameView for GameViewPublic<T>
where
    T: ViewTypes,
{
    type B = T::Button;
}
