use super::{
    ClickHandler, HasClickHandlers, HasMutableColor, RustHandlerRegistration,
    Sprite,
};
use crate::model::Rect;
use crate::view_types::ViewTypes;
use crate::widget;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

lazy_static! {
    static ref handler_registration_id: AtomicU64 = AtomicU64::default();
}

pub trait Button: HasClickHandlers + Send + Sync + 'static {}

widget!(Button<T> {
    sprites {
        outline
    }

    private {
        click_handlers: HashMap<u64, ClickHandler>,
        rect: Rect
    }
});

impl<T> Button for ButtonPublic<T> where T: ViewTypes {}

impl<T> HasClickHandlers for ButtonPublic<T>
where
    T: ViewTypes,
{
    type R = RustHandlerRegistration;

    fn add_click_handler(&self, handler: ClickHandler) -> Self::R {
        let key = handler_registration_id.fetch_add(1, Ordering::Relaxed);

        self.sink.send(move |button| {
            button.add_click_handler(key, handler);
        });

        let copy = self.sink.clone();
        RustHandlerRegistration::new(move || {
            copy.send(move |button| button.remove_click_handler(key))
        })
    }
}

impl<T> ButtonPrivate<T>
where
    T: ViewTypes,
{
    pub fn add_click_handler(&mut self, key: u64, handler: ClickHandler) {
        self.click_handlers.insert(key, handler);
    }

    pub fn remove_click_handler(&mut self, key: u64) {
        self.click_handlers.remove(&key);
    }

    pub fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
        self.outline.set_rect(&self.rect)
    }

    pub fn set_color(&mut self, color: T::Color) {
        self.outline.set_color(color)
    }

    pub fn on_click(&mut self) {
        for handler in self.click_handlers.values() {
            handler.on_click()
        }
    }
}
