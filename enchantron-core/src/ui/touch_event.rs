use super::{Touch, TouchEventType};

#[derive(Debug, Clone, Copy, derive_new::new)]
pub struct TouchEvent {
    pub state: TouchEventType,
    pub touch: Touch,
    pub other_touch_opt: Option<Touch>,
}

impl TouchEvent {
    pub fn start_1(touch: Touch) -> Self {
        TouchEvent::new(TouchEventType::Start, touch, None)
    }

    pub fn start_2(touch_1: Touch, touch_2: Touch) -> Self {
        TouchEvent::new(TouchEventType::Start, touch_1, Some(touch_2))
    }

    pub fn move_1(touch: Touch) -> Self {
        TouchEvent::new(TouchEventType::Move, touch, None)
    }

    pub fn move_2(touch_1: Touch, touch_2: Touch) -> Self {
        TouchEvent::new(TouchEventType::Move, touch_1, Some(touch_2))
    }

    pub fn end_1(touch: Touch) -> Self {
        TouchEvent::new(TouchEventType::End, touch, None)
    }

    pub fn end_2(touch_1: Touch, touch_2: Touch) -> Self {
        TouchEvent::new(TouchEventType::End, touch_1, Some(touch_2))
    }
}
