use super::location_service::{MINIMUM_DISTANCE, MINIMUM_SPEED};
use crate::model::Point;

const MINIMUM_DISTANCE_SQUARED: f64 = MINIMUM_DISTANCE * MINIMUM_DISTANCE;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MovementRequest {
    Stop,
    Maintain,
    MoveToward { target: Point, speed: f64 },
}

impl MovementRequest {
    /// Ensure that the given request is valid based on the configured minimum
    /// speed and minimum distance in the location service
    pub fn sanitize(self, curr_center: &Point) -> Self {
        if let MovementRequest::MoveToward { target, speed } = &self {
            if *speed < MINIMUM_SPEED {
                MovementRequest::Stop
            } else if target.distance_squared_to(curr_center)
                < MINIMUM_DISTANCE_SQUARED
            {
                MovementRequest::Stop
            } else {
                self
            }
        } else {
            self
        }
    }
}
