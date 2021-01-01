use super::{
    Color, HasMutableColor, HasMutableFloatValue, HasMutableLocation,
    HasMutableSize, HasMutableVisibility, Sprite, SpriteGroup, SpriteSource,
};
use crate::model::{ISize, Point, Rect, Size};
use crate::ui::{AnyConsumer, WidgetSelector};
use crate::view_types::ViewTypes;
use crate::widget;
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

widget!(ProgressBar<T> {
    sprites {
        outline,
        bar
    }

    private {
        rect: Rect,
        value: f64
    }
});

impl<T> ProgressBarPrivate<T>
where
    T: ViewTypes,
{
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
        self.send(|mut_self| mut_self.update_progress(new_value))
    }
}
