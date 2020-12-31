use super::NativeView;
use crate::model::{ISize, Rect, Size};
use crate::ui::{
    Color, HandlerRegistration, HasLayoutHandlers, LayoutHandler, ProgressBar,
    ProgressBarPrivate, ProgressBarPublic,
};
use crate::view::AnyConsumer;
use crate::view_impl;
use crate::view_types::ViewTypes;
use std::any::Any;
use std::sync::Arc;
use tokio::sync::mpsc::{channel, Receiver, Sender};

const MAX_WIDTH_FRAC: f64 = 0.8;
const HEIGHT_FRAC: f64 = 0.1;
const BUTTON_ASPECT_RATIO: f64 = 1.618 * 3.;

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

pub trait LoadingView: 'static + Send + Sync + Sized + NativeView {
    type P: ProgressBar;

    fn get_progress_bar(&self) -> Self::P;

    fn transition_to_main_menu_view(&self);
}

view_impl!(LoadingView<T> {
    widgets {
        progress_bar: ProgressBar
    }

    init = init;

    on_layout = on_layout;
});

impl<T> LoadingView for LoadingViewPublic<T>
where
    T: ViewTypes<ProgressBar = ProgressBarPublic<T>>,
{
    type P = T::ProgressBar;

    fn transition_to_main_menu_view(&self) {}

    fn get_progress_bar(&self) -> Self::P {
        self.inner.progress_bar.clone()
    }
}

impl<T> LoadingViewPublic<T>
where
    T: ViewTypes,
{
    pub fn new_loading_view(raw_view: T::NativeView) -> LoadingViewPublic<T> {
        LoadingViewPublic::new(raw_view)
    }
}

impl<T> LoadingViewPrivate<T>
where
    T: ViewTypes,
{
    fn init(&mut self) {
        self.progress_bar
            .set_background_color(T::Color::new(13, 15, 30, 255));
        self.progress_bar
            .set_foreground_color(T::Color::new(103, 90, 140, 200));
    }

    fn on_layout(&mut self, size: Size) {
        let loading_rect = calculate_rect_from_size(size);
        self.progress_bar.set_rect(loading_rect);
    }
}
