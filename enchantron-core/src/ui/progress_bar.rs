use super::{
    HasMutableColor, HasMutableFloatValue, HasMutableLocation, HasMutableSize,
    HasMutableVisibility, Sprite, SpriteGroup, SpriteSource,
};
use crate::model::{Rect, ISize, Size};
use crate::view_types::ViewTypes;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;

trait ProgressBarCommand<T> : FnOnce(&mut ProgressBarImpl<T>) + Send + 'static {}

pub trait ProgressBar: HasMutableFloatValue + Send + Sync + 'static {}

pub struct ProgressBarImpl<T: ViewTypes> {
    inner: Arc<Inner<T>>,
}

struct Inner<T: ViewTypes> {
    outline: T::Sprite,
    bar: T::Sprite,

    rect: Rect,
    value: f64,

    sender: Sender<Box<dyn ProgressBarCommand<T>>>
}

impl<T> Clone for ProgressBarImpl<T>
where
    T: ViewTypes,
{
    fn clone(&self) -> Self {
        ProgressBarImpl {
            inner: self.inner.clone(),
        }
    }
}

impl<T> ProgressBarImpl<T>
where
    T: ViewTypes,
{
    pub fn new(
        sprite_source: &impl SpriteSource<T = T::Texture, S = T::Sprite>,

    ) -> ProgressBarImpl<T> {
        let outline = sprite_source.create_sprite();
        let bar = sprite_source.create_sprite();

        ProgressBarImpl {
            inner: Arc::new(Inner {
                outline,
                bar,
                rect: Default::default(),
                value: Default::default(),
            }),
        }
    }

    pub fn layout(&self, size: ISize) {
    }
}

impl<T> ProgressBar for ProgressBarImpl<T> where T: ViewTypes {}

impl<T> HasMutableFloatValue for ProgressBarImpl<T>
where
    T: ViewTypes,
{
    fn set_value(&self, new_value: f64) {
        self.sender.try_send(|progress_bar| {
            progress_bar.
        })
    }
}
