use super::{
    DragPoint, Finger, Finger::*, Touch, TouchEvent, TouchEventType::*,
};
use crate::model::Point;
use enum_map::EnumMap;

fn calculate_shift(curr_drag: &DragPoint, prev_drag: &DragPoint) -> Point {
    &prev_drag.global_point - &curr_drag.global_point
}

#[allow(clippy::many_single_char_names)]
fn calculate_shift_and_scale(
    prev_drag_1: &DragPoint,
    prev_drag_2: &DragPoint,
    curr_drag_1: &DragPoint,
    curr_drag_2: &DragPoint,
) -> (Point, f64) {
    let n = 2.0;
    let a1 = curr_drag_1.global_point.x;
    let a2 = curr_drag_2.global_point.x;
    let b1 = curr_drag_1.global_point.y;
    let b2 = curr_drag_2.global_point.y;
    let c1 = prev_drag_1.global_point.x;
    let c2 = prev_drag_2.global_point.x;
    let d1 = prev_drag_1.global_point.y;
    let d2 = prev_drag_2.global_point.y;

    let u = a1 * a1 + a2 * a2 + b1 * b1 + b2 * b2;
    let v = a1 + a2;
    let w = b1 + b2;
    let x = c1 + c2;
    let y = d1 + d2;
    let ac = a1 * c1 + a2 * c2;
    let bd = b1 * d1 + b2 * d2;

    let gr = 1.0 / (n * n * u - n * v * v - n * w * w);

    let s = gr * (n * n * (ac + bd) - n * (v * x + w * y));
    let dx =
        gr * (-n * v * ac - n * v * bd + n * u * x - w * w * x + v * w * y);
    let dy =
        gr * (-n * w * ac - n * w * bd + n * u * y - v * v * y + v * w * x);

    (Point::new(dx, dy), s)
}

pub enum PanZoomEvent {
    Move(Point),
    MoveAndScale(Point, f64),
}

#[derive(Debug, Default)]
pub struct PanZoomTracker {
    touches: EnumMap<Finger, Option<Touch>>,
    touch_count: usize,
}

impl PanZoomTracker {
    pub fn to_pan_zoom_event(
        &mut self,
        touch_event: TouchEvent,
    ) -> Option<PanZoomEvent> {
        match touch_event {
            TouchEvent {
                state: Start,
                touch,
                other_touch_opt: None,
            } => self.on_one_drag_start(touch),
            TouchEvent {
                state: Start,
                touch,
                other_touch_opt: Some(other_touch),
            } => self.on_two_drags_start(touch, other_touch),
            TouchEvent {
                state: Move,
                touch,
                other_touch_opt: None,
            } => self.on_one_drag_move(touch),
            TouchEvent {
                state: Move,
                touch,
                other_touch_opt: Some(other_touch),
            } => self.on_two_drags_move(touch, other_touch),
            TouchEvent {
                state: End,
                touch,
                other_touch_opt: None,
            } => self.on_one_drag_end(touch),
            TouchEvent {
                state: End,
                touch,
                other_touch_opt: Some(other_touch),
            } => self.on_two_drags_end(touch, other_touch),
        }
    }

    fn on_one_drag_start(&mut self, touch: Touch) -> Option<PanZoomEvent> {
        self.touches[touch.finger] = Some(touch);
        self.touch_count += 1;
        None
    }

    fn on_two_drags_start(
        &mut self,
        touch_1: Touch,
        touch_2: Touch,
    ) -> Option<PanZoomEvent> {
        self.touches[touch_1.finger] = Some(touch_1);
        self.touches[touch_2.finger] = Some(touch_2);
        self.touch_count += 2;
        None
    }

    fn on_one_drag_move(&mut self, moved_touch: Touch) -> Option<PanZoomEvent> {
        if self.touch_count == 1 {
            let prev_touch = self.touches[moved_touch.finger].as_mut().unwrap();
            let shift = calculate_shift(&moved_touch.point, &prev_touch.point);
            *prev_touch = moved_touch;
            Some(PanZoomEvent::Move(shift))
        } else {
            let prev_touch =
                self.touches[moved_touch.finger].as_ref().copied().unwrap();
            let unmoved_finger = if moved_touch.finger == Finger1 {
                Finger2
            } else {
                Finger1
            };
            let unmoved_touch =
                self.touches[unmoved_finger].as_ref().copied().unwrap();

            let (shift, scale) = calculate_shift_and_scale(
                &prev_touch.point,
                &unmoved_touch.point,
                &moved_touch.point,
                &unmoved_touch.point,
            );

            self.touches[moved_touch.finger] = Some(moved_touch);
            Some(PanZoomEvent::MoveAndScale(shift, scale))
        }
    }

    fn on_two_drags_move(
        &mut self,
        moved_touch_1: Touch,
        moved_touch_2: Touch,
    ) -> Option<PanZoomEvent> {
        let prev_touch_1 = self.touches[moved_touch_1.finger]
            .as_ref()
            .copied()
            .unwrap();
        let prev_touch_2 = self.touches[moved_touch_2.finger]
            .as_ref()
            .copied()
            .unwrap();

        let (shift, scale) = calculate_shift_and_scale(
            &prev_touch_1.point,
            &prev_touch_2.point,
            &moved_touch_1.point,
            &moved_touch_2.point,
        );

        self.touches[moved_touch_1.finger] = Some(moved_touch_1);
        self.touches[moved_touch_2.finger] = Some(moved_touch_2);

        Some(PanZoomEvent::MoveAndScale(shift, scale))
    }

    fn on_one_drag_end(&mut self, touch: Touch) -> Option<PanZoomEvent> {
        self.touches[touch.finger] = None;
        self.touch_count -= 1;
        None
    }

    fn on_two_drags_end(&mut self, _: Touch, _: Touch) -> Option<PanZoomEvent> {
        self.touches[Finger1] = None;
        self.touches[Finger2] = None;
        self.touch_count -= 2;
        None
    }
}
