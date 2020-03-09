use super::{DragEventType::*, DragPoint};
use crate::event::DragEvent;
use crate::model::Point;

fn calcualte_shift(prev_drag: &DragPoint, curr_drag: &DragPoint) -> Point {
    &curr_drag.global_point - &prev_drag.global_point
}

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

pub enum DragTrackerEvent {
    Move(Point),
    MoveAndScale(Point, f64),
}

#[derive(Debug, Default)]
pub struct DragTracker {
    drag_1: Option<DragPoint>,
    drag_2: Option<DragPoint>,
}

impl DragTracker {
    pub fn on_drag_event(
        &mut self,
        drag_event: DragEvent,
    ) -> Option<DragTrackerEvent> {
        match drag_event {
            DragEvent {
                state: Start,
                drag_point_1,
                drag_point_2_opt: None,
            } => self.on_one_drag_start(drag_point_1),
            DragEvent {
                state: Start,
                drag_point_1,
                drag_point_2_opt: Some(drag_point_2),
            } => self.on_two_drags_start(drag_point_1, drag_point_2),
            DragEvent {
                state: Move,
                drag_point_1,
                drag_point_2_opt: None,
            } => self.on_one_drag_move(drag_point_1),
            DragEvent {
                state: Move,
                drag_point_1,
                drag_point_2_opt: Some(drag_point_2),
            } => self.on_two_drags_move(drag_point_1, drag_point_2),
            DragEvent {
                state: End,
                drag_point_1,
                drag_point_2_opt: None,
            } => self.on_one_drag_end(drag_point_1),
            DragEvent {
                state: End,
                drag_point_1,
                drag_point_2_opt: Some(drag_point_2),
            } => self.on_two_drags_end(drag_point_1, drag_point_2),
        }
    }

    fn on_one_drag_start(
        &mut self,
        new_drag_point: DragPoint,
    ) -> Option<DragTrackerEvent> {
        if self.drag_1.is_none() {
            self.drag_1 = Some(new_drag_point);
        } else {
            self.drag_2 = Some(new_drag_point);
        }

        None
    }

    fn on_two_drags_start(
        &mut self,
        new_drag_point_1: DragPoint,
        new_drag_point_2: DragPoint,
    ) -> Option<DragTrackerEvent> {
        self.drag_1 = Some(new_drag_point_1);
        self.drag_2 = Some(new_drag_point_2);

        None
    }

    fn on_one_drag_move(
        &mut self,
        moved_drag_point: DragPoint,
    ) -> Option<DragTrackerEvent> {
        Some(match (&mut self.drag_1, &mut self.drag_2) {
            (Some(current_drag), None) | (None, Some(current_drag)) => {
                let shift = calcualte_shift(&moved_drag_point, current_drag);
                *current_drag = moved_drag_point;
                DragTrackerEvent::Move(shift)
            }
            (Some(drag_1), Some(drag_2)) => {
                let (current_drag, other_drag) =
                    if drag_1.drag_id == moved_drag_point.drag_id {
                        (drag_1, drag_2)
                    } else {
                        (drag_2, drag_1)
                    };

                let (shift, scale) = calculate_shift_and_scale(
                    current_drag,
                    other_drag,
                    &moved_drag_point,
                    other_drag,
                );

                *current_drag = moved_drag_point;
                DragTrackerEvent::MoveAndScale(shift, scale)
            }
            _ => panic!("Invalid drag operation"),
        })
    }

    fn on_two_drags_move(
        &mut self,
        moved_drag_point_1: DragPoint,
        moved_drag_point_2: DragPoint,
    ) -> Option<DragTrackerEvent> {
        todo!()
    }

    fn on_one_drag_end(
        &mut self,
        ended_drag_point: DragPoint,
    ) -> Option<DragTrackerEvent> {
        if self
            .drag_1
            .as_ref()
            .filter(|point| point.drag_id == ended_drag_point.drag_id)
            .is_some()
        {
            self.drag_1 = None
        } else {
            self.drag_2 = None
        }

        None
    }

    fn on_two_drags_end(
        &mut self,
        ended_drag_point_1: DragPoint,
        ended_drag_point_2: DragPoint,
    ) -> Option<DragTrackerEvent> {
        todo!()
    }
}
