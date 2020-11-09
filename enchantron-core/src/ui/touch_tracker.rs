use super::{
    Finger, Finger::*, RawTouch, Touch, TouchEvent, TouchEventType::*,
    ViewportInfo,
};
use crate::event::RawTouchEvent;
use enum_map::EnumMap;

#[derive(Debug, Default)]
pub struct TouchTracker {
    touches: EnumMap<Finger, Option<(i64, Touch)>>,
}

impl TouchTracker {
    fn get_finger_for_id(&self, to_find: i64) -> Finger {
        self.touches[Finger1]
            .filter(|(id, _)| *id == to_find)
            .map(|_| Finger1)
            .unwrap_or(Finger2)
    }

    pub fn to_touch_event(
        &mut self,
        raw_event: &RawTouchEvent,
        viewport_info: &ViewportInfo,
    ) -> TouchEvent {
        match raw_event {
            RawTouchEvent {
                state: Start,
                touch,
                other_touch_opt: None,
            } => self.on_one_touch_start(touch, viewport_info),
            RawTouchEvent {
                state: Start,
                touch,
                other_touch_opt: Some(other_touch),
            } => self.on_two_touch_start(touch, other_touch, viewport_info),
            RawTouchEvent {
                state: Move,
                touch,
                other_touch_opt: None,
            } => self.on_one_touch_move(touch, viewport_info),
            RawTouchEvent {
                state: Move,
                touch,
                other_touch_opt: Some(other_touch),
            } => self.on_two_touch_move(touch, other_touch, viewport_info),
            RawTouchEvent {
                state: End,
                touch,
                other_touch_opt: None,
            } => self.on_one_touch_end(touch, viewport_info),
            RawTouchEvent {
                state: End,
                touch,
                other_touch_opt: Some(other_touch),
            } => self.on_two_touch_end(touch, other_touch, viewport_info),
        }
    }

    fn on_one_touch_start(
        &mut self,
        touch: &RawTouch,
        viewport_info: &ViewportInfo,
    ) -> TouchEvent {
        let touch_id = touch.touch_id;

        let touch = if self.touches[Finger1].is_none() {
            Touch::new(Finger1, touch, viewport_info)
        } else {
            Touch::new(Finger2, touch, viewport_info)
        };

        self.touches[touch.finger] = Some((touch_id, touch));

        TouchEvent::start_1(touch)
    }

    fn on_two_touch_start(
        &mut self,
        touch_1: &RawTouch,
        touch_2: &RawTouch,
        viewport_info: &ViewportInfo,
    ) -> TouchEvent {
        let touch_id_1 = touch_1.touch_id;
        let touch_id_2 = touch_2.touch_id;

        let touch_1 = Touch::new(Finger1, touch_1, viewport_info);
        let touch_2 = Touch::new(Finger2, touch_2, viewport_info);

        self.touches[touch_1.finger] = Some((touch_id_1, touch_1));
        self.touches[touch_2.finger] = Some((touch_id_2, touch_2));

        TouchEvent::start_2(touch_1, touch_2)
    }

    fn on_one_touch_move(
        &mut self,
        raw_touch: &RawTouch,
        viewport_info: &ViewportInfo,
    ) -> TouchEvent {
        let finger = self.get_finger_for_id(raw_touch.touch_id);
        let touch = self.touches[finger]
            .as_mut()
            .unwrap()
            .1
            .update(raw_touch, viewport_info);

        TouchEvent::move_1(touch)
    }

    fn on_two_touch_move(
        &mut self,
        raw_touch_1: &RawTouch,
        raw_touch_2: &RawTouch,
        viewport_info: &ViewportInfo,
    ) -> TouchEvent {
        let finger_1 = self.get_finger_for_id(raw_touch_1.touch_id);
        let finger_2 = self.get_finger_for_id(raw_touch_2.touch_id);
        let touch_1 = self.touches[finger_1]
            .as_mut()
            .unwrap()
            .1
            .update(raw_touch_1, viewport_info);
        let touch_2 = self.touches[finger_2]
            .as_mut()
            .unwrap()
            .1
            .update(raw_touch_2, viewport_info);

        TouchEvent::move_2(touch_1, touch_2)
    }

    fn on_one_touch_end(
        &mut self,
        raw_touch: &RawTouch,
        viewport_info: &ViewportInfo,
    ) -> TouchEvent {
        let finger = self.get_finger_for_id(raw_touch.touch_id);
        let touch = self.touches[finger]
            .as_mut()
            .unwrap()
            .1
            .update(raw_touch, viewport_info);

        self.touches[finger] = None;

        TouchEvent::end_1(touch)
    }

    fn on_two_touch_end(
        &mut self,
        raw_touch_1: &RawTouch,
        raw_touch_2: &RawTouch,
        viewport_info: &ViewportInfo,
    ) -> TouchEvent {
        let finger_1 = self.get_finger_for_id(raw_touch_1.touch_id);
        let finger_2 = self.get_finger_for_id(raw_touch_2.touch_id);
        let touch_1 = self.touches[finger_1]
            .as_mut()
            .unwrap()
            .1
            .update(raw_touch_1, viewport_info);
        let touch_2 = self.touches[finger_2]
            .as_mut()
            .unwrap()
            .1
            .update(raw_touch_2, viewport_info);

        self.touches[Finger1] = None;
        self.touches[Finger2] = None;

        TouchEvent::move_2(touch_1, touch_2)
    }
}
