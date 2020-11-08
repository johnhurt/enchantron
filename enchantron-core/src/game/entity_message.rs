use crate::model::IPoint;

pub enum EntityMessage {
    EnteredViewport,
    ExitedViewport,
    GoalSet(IPoint),
}
