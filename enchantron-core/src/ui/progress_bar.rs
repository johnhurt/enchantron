use super::{
    Color, HasMutableColor, HasMutableFloatValue, HasMutableLocation,
    HasMutableSize, HasMutableVisibility, Sprite, SpriteGroup, SpriteSource,
};
use crate::model::{ISize, Point, Rect, Size};
use crate::view::{AnyConsumer, WidgetSelector};
use crate::view_types::ViewTypes;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;

/// Calculate the rect for the bar part of the progress bar given the outline
/// and progress value (0 - 1)
fn get_bar_rect(outline_rect: &Rect, progress: f64) -> Rect {
    let height = outline_rect.size.height / 3.;
    let top_left = outline_rect.top_left + Point::new(height, height);
    let width = progress * (outline_rect.size.width - 2. * height);

    Rect {
        top_left,
        size: Size::new(width, height),
    }
}

pub trait ProgressBar: HasMutableFloatValue + Send + Sync + 'static {}

pub struct ProgressBarPublic<T: ViewTypes> {
    selector: WidgetSelector<ProgressBarPrivate<T>>,
    sender: Arc<Sender<AnyConsumer>>,
}

pub struct ProgressBarPrivate<T: ViewTypes> {
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
            selector: self.selector.clone(),
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
        sender: Arc<Sender<AnyConsumer>>,
        selector: WidgetSelector<ProgressBarPrivate<T>>,
    ) -> ProgressBarPrivate<T> {
        let outline = sprite_source.create_sprite();
        let bar = sprite_source.create_sprite();

        outline.set_color(T::Color::new(0, 0, 0, 255));
        bar.set_color(T::Color::new(200, 200, 200, 255));

        ProgressBarPrivate {
            outline,
            bar,
            rect: Default::default(),
            value: Default::default(),
            public: ProgressBarPublic { sender, selector },
        }
    }

    pub fn public(&self) -> ProgressBarPublic<T> {
        self.public.clone()
    }

    pub fn set_foreground_color(&self, color: T::Color) {
        self.bar.set_color(color);
    }

    pub fn set_background_color(&self, color: T::Color) {
        self.outline.set_color(color);
    }

    pub fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
        self.render();
    }

    pub fn update_progress(&mut self, progress: f64) {
        self.value = progress;
        self.render();
    }

    fn render(&self) {
        let bar_rect = get_bar_rect(&self.rect, self.value);
        self.outline.set_rect(&self.rect);
        self.bar.set_rect(&bar_rect);
    }
}

impl<T> ProgressBar for ProgressBarPublic<T> where T: ViewTypes {}

impl<T> HasMutableFloatValue for ProgressBarPublic<T>
where
    T: ViewTypes,
{
    fn set_value(&self, new_value: f64) {
        let copy = self.clone();
        let _ = self.sender.try_send(Box::new(move |any| {
            (copy.selector)(any).update_progress(new_value)
        }));
    }
}
