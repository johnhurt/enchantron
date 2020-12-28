use crate::view::{LoadingView, NativeView};

pub trait TransitionService: Send + Sync + 'static {
    type NV: NativeView;
    type LV: LoadingView;

    fn transition_to_native_view(&self, view: &Self::NV, drop_current: bool);
    fn transition_to_loading_view(&self, view: &Self::LV, drop_current: bool);
}
