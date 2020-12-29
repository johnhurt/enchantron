use super::{
    HasMutableColor, HasMutableFloatValue, HasMutableLocation, HasMutableSize,
    HasMutableVisibility, Sprite, SpriteGroup, SpriteSource,
};
use crate::model::{ISize, Rect, Size};
use crate::view_types::ViewTypes;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;

type ProgressBarCommand<T, V> =
    Box<dyn FnOnce(&mut ProgressBarPrivate<T, V>) + Send + 'static>;

type ProgressBarSelector<T, V> =
    Box<dyn Fn(&mut V) -> &mut ProgressBarPrivate<T, V> + Send + 'static>;

pub trait ProgressBar: HasMutableFloatValue + Send + Sync + 'static {}

pub struct ProgressBarPublic<T: ViewTypes, V> {
    selector: ProgressBarSelector<T,V>,
    sender: Arc<Sender<(ProgressBarSelector<T,V>, ProgressBarCommand<T,V>)>>,
}

pub struct ProgressBarPrivate<T: ViewTypes, V> {
    outline: T::Sprite,
    bar: T::Sprite,

    rect: Rect,
    value: f64,

    public: ProgressBarPublic<T>,
}

impl<T> Clone for ProgressBarPublic<T>
where
    T: ViewTypes,
{
    fn clone(&self) -> Self {
        ProgressBarPublic {
            sender: self.sender.clone(),
        }
    }
}

impl<T> ProgressBarPrivate<T>
where
    T: ViewTypes,
{
    pub fn new(
        sprite_source: &impl SpriteSource<T = T::Texture, S = T::Sprite>,
        sender: Arc<Sender<ProgressBarCommand<T>>>,
    ) -> ProgressBarPrivate<T> {
        let outline = sprite_source.create_sprite();
        let bar = sprite_source.create_sprite();

        ProgressBarPrivate {
            outline,
            bar,
            rect: Default::default(),
            value: Default::default(),
            public: ProgressBarPublic { sender },
        }
    }

    pub fn get_public(&self) -> ProgressBarPublic<T> {
        self.public.clone()
    }

    pub fn layout(&mut self, size: ISize) {}

    pub fn update_progress(&mut self, progress: f64) {}

    fn render(&self) {}
}

impl<T> ProgressBar for ProgressBarPublic<T> where T: ViewTypes {}

impl<T> HasMutableFloatValue for ProgressBarPublic<T>
where
    T: ViewTypes,
{
    fn set_value(&self, new_value: f64) {
        let _ = self.sender.try_send(Box::new(|progress_bar| {
            progress_bar.update_progress(new_value)
        }));
    }
}
