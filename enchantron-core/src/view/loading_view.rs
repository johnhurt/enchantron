use super::NativeView;
use crate::view_impl;
use crate::view_types::ViewTypes;

pub trait LoadingView: 'static + Send + Sync + Sized + NativeView {
    fn transition_to_main_menu_view(&self);
}

view_impl!(LoadingViewImpl : LoadingView {
});

impl<T> LoadingView for LoadingViewImpl<T>
where
    T: ViewTypes,
{
    fn transition_to_main_menu_view(&self) {}
}
