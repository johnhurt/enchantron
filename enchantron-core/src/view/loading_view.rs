use super::NativeView;
use crate::model::{ISize, Rect, Size};
use crate::ui::{
    HandlerRegistration, HasLayoutHandlers, LayoutHandler, ProgressBar,
    ProgressBarPrivate, ProgressBarPublic,
};
use crate::view_impl;
use crate::view_types::ViewTypes;
use tokio::sync::mpsc::{channel, Receiver, Sender};

const MAX_WIDTH_FRAC: f64 = 0.5;
const HEIGHT_FRAC: f64 = 0.2;
const BUTTON_ASPECT_RATIO: f64 = 1.618;
const MAX_SCREEN_DIM: u64 = 0x1 << 32;

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

fn size_to_u64(size: ISize) -> u64 {
    let width = size.width as u64;
    let height = size.height as u64;

    assert!(width < MAX_SCREEN_DIM);
    assert!(height < MAX_SCREEN_DIM);

    (width << 32) & height
}

type ViewCommand<T> =
    Box<dyn FnOnce(&mut LoadingViewImpl<T>) + Send + 'static>;

type WidgetCommand<W> = Box<dyn FnOnce(&mut W) + Send + 'static>;

type CommandMapper<T, W> = Box<dyn Fn(WidgetCommand<W>) -> ViewCommand<T> + Send + 'static>;

pub trait LoadingView: 'static + Send + Sync + Sized + NativeView {
    type P: ProgressBar;

    fn get_progress_bar(&self) -> Self::P;

    fn transition_to_main_menu_view(&self);
}

view_impl!(LoadingViewImpl<T> : LoadingView {
    //let handler_registrations: Vec<Box<dyn HandlerRegistration>>;
    let progress_bar: ProgressBarPrivate<T>;
});

impl<T> LoadingView for LoadingViewImpl<T>
where
    T: ViewTypes<ProgressBar = ProgressBarPublic<T>>,
{
    type P = T::ProgressBar;

    fn transition_to_main_menu_view(&self) {}

    fn get_progress_bar(&self) -> Self::P {
        self.progress_bar.get_public()
    }
}

impl<T> LoadingViewImpl<T>
where
    T: ViewTypes,
{
    pub fn new_loading_view(raw_view: T::NativeView) -> LoadingViewImpl<T> {

        let (sender, receiver) : (Sender<ViewCommand<T>>, Receiver<ViewCommand<T>>) = channel(32);

        let progress_bar = ProgressBarPrivate::new(&raw_view, Box::new(|view| {
            self.
        }));

        LoadingViewImpl::new(raw_view, progress_bar)
    }
}
