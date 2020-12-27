use super::NativeView;
use crate::view_impl;
use crate::view_types::ViewTypes;

pub trait GameView: NativeView + Sync + Send + 'static {}

view_impl!(GameViewImpl<T> : GameView {
});

impl<T> GameView for GameViewImpl<T> where T: ViewTypes {}
