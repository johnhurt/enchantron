use super::{DragPoint, Tap, Touch, TouchEvent, TouchEventType};

const MAX_TAP_DIST_SQR_SUM: f64 = 10.;

#[derive(Debug, Clone, Copy, derive_new::new)]
pub struct TapEvent {
    pub tap: Tap,
    pub other_tap_opt: Option<Tap>,
}

impl TapEvent {
    pub fn from_touch_event(touch_event: TouchEvent) -> Option<TapEvent> {
        match touch_event {
            TouchEvent {
                state: TouchEventType::End,
                touch:
                    Touch {
                        point,
                        click_count,
                        move_dist_sqr_sum,
                        ..
                    },
                other_touch_opt: None,
            } if move_dist_sqr_sum <= MAX_TAP_DIST_SQR_SUM => {
                Some(TapEvent::new(Tap::new(click_count, point), None))
            }
            TouchEvent {
                state: TouchEventType::End,
                touch:
                    Touch {
                        point,
                        click_count,
                        move_dist_sqr_sum,
                        ..
                    },
                other_touch_opt:
                    Some(Touch {
                        point: other_point,
                        click_count: other_click_count,
                        move_dist_sqr_sum: other_move_dist_sqr_sum,
                        ..
                    }),
            } => {
                match (
                    move_dist_sqr_sum <= MAX_TAP_DIST_SQR_SUM,
                    other_move_dist_sqr_sum <= MAX_TAP_DIST_SQR_SUM,
                ) {
                    (true, true) => Some(TapEvent::new(
                        Tap::new(click_count, point),
                        Some(Tap::new(other_click_count, other_point)),
                    )),
                    (true, false) => {
                        Some(TapEvent::new(Tap::new(click_count, point), None))
                    }
                    (false, true) => Some(TapEvent::new(
                        Tap::new(other_click_count, other_point),
                        None,
                    )),
                    _ => None,
                }
            }
            _ => None,
        }
    }
}
