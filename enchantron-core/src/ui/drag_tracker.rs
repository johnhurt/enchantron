use super::{DragEventType::*, DragPoint};
use crate::event::DragEvent;
use crate::model::Point;

pub enum DragTrackerEvent {
    Move(Point),
    MoveAndScale(Point, f64),
}

#[derive(Debug, Default)]
pub struct DragTracker {
    drag_count: usize,
    drag_1: Option<DragPoint>,
    drag_2: Option<DragPoint>,
}

impl DragTracker {
    pub fn on_drag_event(
        &mut self,
        drag_event: DragEvent,
    ) -> Option<DragTrackerEvent> {
        match drag_event {
            DragPoint {
                state: Start,
                drag_point_1,
                drag_point2_opt: None,
            } => self.on_one_drag_start(drag_point_1),
            DragPoint {
                state: Start,
                drag_point_1,
                drag_point2_opt: Some(drag_point_2),
            } => self.on_two_drag_start(drag_point_1, drag_point_2),
            _ => todo!(),
        }
    }

    fn on_one_drag_start(
        &mut self,
        new_drag_point: DragPoint,
    ) -> Option<DragTrackerEvent> {
        match self.drag_count {
            2 => debug!("Touch {} rejected", new_drag_point.drag_id),
            1 => self.drag_point_2 = Some(new_drag_point),
            0 => self.drag_point_1 = Some(new_drag_point),
            _ => panic!("Inavlid drag count: {}", self.drag_count),
        }

        None
    }

    fn on_two_drag_start(
        &mut self,
        new_drag_point_1: DragPoint,
        new_drag_point_2: DragPoint,
    ) -> Option<DragTrackerEvent> {
        match self.drag_count {
            2 => debug!(
                "Touches: {} & {} rejected",
                new_drag_point_1.drag_id, new_drag_point_2.drag_id
            ),
            1 => {
                debug!("Touch {} rejected", new_drag_point_2.drag_id);
                self.drag_point_2 = Some(new_drag_point_1);
            }
            0 => {
                self.drag_point_1 = Some(new_drag_point_1);
                self.drag_point_2 = Some(new_drag_point_2)
            }
            _ => panic!("Inavlid drag count: {}", self.drag_count),
        }

        None
    }

    fn on_one_drag_move(
        &mut self,
        drag_point: &DragPoint,
    ) -> Option<DragTrackerEvent> {
        todo!()
    }

    fn on_two_drag_move(
        &mut self,
        drag_point_1: &DragPoint,
        drag_point_2: &DragPoint,
    ) -> Option<DragTrackerEvent> {
        todo!()
    }

    fn on_one_drag_end(
        &mut self,
        drag_point: &DragPoint,
    ) -> Option<DragTrackerEvent> {
        todo!()
    }

    fn on_two_drag_end(
        &mut self,
        drag_point_1: &DragPoint,
        drag_point_2: &DragPoint,
    ) -> Option<DragTrackerEvent> {
        todo!()
    }
}
