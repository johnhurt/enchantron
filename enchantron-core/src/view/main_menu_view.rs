use super::NativeView;
use crate::view_impl;
use crate::view_types::ViewTypes;

pub trait MainMenuView: 'static + Sized + Send + Sync + NativeView {
    fn transition_to_game_view(&self);
}

view_impl!(MainMenuViewImpl : MainMenuView {
});

impl<T> MainMenuView for MainMenuViewImpl<T>
where
    T: ViewTypes,
{
    fn transition_to_game_view(&self) {}
}
