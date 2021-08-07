use crate::model::Point;

use super::Entity;

#[derive(Debug, derive_new::new)]
pub struct CollisionPrediction {
    time: f64,
    entity: Entity,
}

#[derive(Debug, derive_new::new)]
pub struct LocationWriteResponse {
    next_update_time: f64,
    collision: Option<CollisionPrediction>,
}
