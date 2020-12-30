use super::NativeView;
use crate::view_impl;
use crate::view_types::ViewTypes;
use std::marker::PhantomData;

pub trait MainMenuView: 'static + Sized + Send + Sync + NativeView {
    fn transition_to_game_view(&self);
}

view_impl!(MainMenuView<T> {
    private {
        _phantom_t : PhantomData<T>
    }
});

impl<T> MainMenuView for MainMenuViewPublic<T>
where
    T: ViewTypes,
{
    fn transition_to_game_view(&self) {}
}
