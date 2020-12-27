use super::NativeView;
use crate::ui::ProgressBar;
use crate::view_impl;
use crate::view_types::ViewTypes;

pub trait LoadingView: 'static + Send + Sync + Sized + NativeView {
    fn transition_to_main_menu_view(&self);
}

view_impl!(new_loading_view<T>() -> LoadingView {
    let progress_bar: ProgressBar<T>;
});

impl<T> LoadingView for LoadingViewImpl<T>
where
    T: ViewTypes,
{
    fn transition_to_main_menu_view(&self) {}
}

impl<T> LoadingViewImpl<T>
where
    T: ViewTypes,
{
    pub fn new_loading_view(raw_view: T::NativeView) -> LoadingViewImpl<T> {
        let progress_bar = ProgressBar::new(&raw_view);

        LoadingViewImpl::new(raw_view, progress_bar)
    }
}
