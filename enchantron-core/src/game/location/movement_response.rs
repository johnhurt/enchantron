use crate::game::Entity;
use crate::model::Point;

#[derive(Debug)]
pub enum MovementResponse {
    Stopped { center: Point },
    ArrivalPredicted { time: f64 },
    MaintenanceNeeded { time: f64 },
    CollisionPredicted { time: f64, entity: Entity },
    CollisionOccurred { entity: Entity },
}
