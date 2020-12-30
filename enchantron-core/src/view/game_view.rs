use super::NativeView;
use crate::view_impl;
use crate::view_types::ViewTypes;
use std::marker::PhantomData;

pub trait GameView: NativeView + Sync + Send + 'static {}

view_impl!(GameView<T> {
    private {
        _phantom_t : PhantomData<T>
    }
});

impl<T> GameView for GameViewPublic<T> where T: ViewTypes {}
