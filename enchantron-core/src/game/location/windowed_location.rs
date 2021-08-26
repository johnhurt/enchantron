use super::{
    location_service::MAX_SECS_BETWEEN_REINSERTS,
    location_service::MIN_SECS_BETWEEN_REINSERTS, MovementResponse,
};
use crate::game::Entity;
use crate::model::{IRect, ISize, Point, Rect};

#[derive(Debug, Copy, Clone, derive_new::new)]
pub struct Movement {
    pub ref_time: f64,
    pub destination: Point,
    pub velocity: Point,
    pub max_valid_time: f64,
}

#[derive(Debug, Copy, Clone)]
pub struct WindowedLocation {
    pub ref_center: Point,
    pub radius: f64,
    pub max_speed: f64,
    pub movement: Option<Movement>,
    pub entity: Entity,
    pub window: IRect,
}

impl WindowedLocation {
    pub fn new(
        entity: Entity,
        center: Point,
        radius: f64,
        max_speed: f64,
    ) -> Self {
        let mut result = WindowedLocation {
            entity,
            ref_center: center,
            radius,
            max_speed,
            movement: None,
            window: IRect::default(),
        };

        result.recenter_window();

        result
    }

    /// Check to see if this window contains the center it represents
    pub fn check_window_contains_location(&self) -> bool {
        self.window.contains_point(&self.ref_center.floor())
    }

    /// Move the window for this windowed pointer to be the rectangle around the
    /// actual location
    pub fn recenter_window(&mut self) {
        let dist = self.radius + MAX_SECS_BETWEEN_REINSERTS * self.max_speed;

        self.window = IRect {
            top_left: self.ref_center.floor(),
            size: ISize::new(1, 1),
        };

        self.window = self.window.expanded_by(dist.ceil() as usize);
    }

    /// Get the topmost and leftmost point that the center can be inside the
    /// window.
    pub fn center_limit_rect(&self) -> Rect {
        let mut result = self.window.as_rect();
        result.top_left.x += self.radius;
        result.top_left.y += self.radius;
        result.size.width -= 2. * self.radius;
        result.size.height -= 2. * self.radius;
        result
    }

    /// Get the time until any part of the entity exits the window for the
    /// given movement.
    ///
    /// Note - the movement given here is assumed to constructed such that the
    ///         effective speed is >= MINIMUM_SPEED
    pub fn time_until_boundary_for(&self, movement: &Movement) -> f64 {
        let center_limit_rect = self.center_limit_rect();
        let offset_from_limit = self.ref_center - center_limit_rect.top_left;

        let ref velocity = movement.velocity;

        let x_time = if velocity.x < 0. {
            offset_from_limit.x / velocity.x
        } else if velocity.x > 0. {
            (center_limit_rect.size.width - offset_from_limit.x) / velocity.x
        } else {
            f64::MAX
        };

        let y_time = if velocity.y < 0. {
            offset_from_limit.y / velocity.y
        } else if velocity.y > 0. {
            (center_limit_rect.size.width - offset_from_limit.y) / velocity.y
        } else {
            f64::MAX
        };

        x_time.min(y_time)
    }

    /// Since the location is stored at a specific time and with a fixed
    /// velocity, we need to know the time we are measuring the position at
    pub fn get_center_at_time(&self, time: f64) -> Point {
        self.ref_center
            + self
                .movement
                .map(|movement| movement.position_offset_at(time))
                .unwrap_or_default()
    }

    /// Update this entity's movement based on the given destination, speed, and
    /// time. Return true if the outer window for this location was changed, and
    /// thus needs to be reinserted into the r tree.
    ///
    /// Note - The speed (|velocity|) given to this method is assumed to be
    ///         greater than the MINIMUM_SPEED
    pub fn upsert_movement(
        &mut self,
        new_ref_time: f64,
        new_ref_center: Point,
        new_target: Point,
        new_velocity: Point,
        time_to_target: f64,
    ) -> (bool, MovementResponse) {
        self.ref_center = new_ref_center;

        let mut new_movement =
            Movement::new(new_ref_time, new_target, new_velocity, 0.);
        let mut time_to_boundary = self.time_until_boundary_for(&new_movement);

        let needs_reinsert = time_to_boundary < MIN_SECS_BETWEEN_REINSERTS;

        if needs_reinsert {
            self.recenter_window();
            time_to_boundary = self.time_until_boundary_for(&new_movement);
        }

        // if the entity will reach the target before it reaches the boundary,
        // then the movement is valid up to the time it reaches the target, and
        // no update/maintenance is needed until after the valid time.
        let response = if time_to_target < time_to_boundary {
            new_movement.max_valid_time = new_ref_time + time_to_boundary;
            MovementResponse::ArrivalPredicted {
                time: new_movement.max_valid_time,
            }
        } else {
            // Otherwise, the movement information is valid until the entity
            // reaches the boundary, but maintenance updates should be done
            // slightly before then.
            new_movement.max_valid_time = new_ref_time + time_to_boundary;
            let next_update_time =
                new_movement.max_valid_time - MIN_SECS_BETWEEN_REINSERTS;
            MovementResponse::MaintenanceNeeded {
                time: new_movement.max_valid_time,
            }
        };

        self.movement = Some(new_movement);

        (needs_reinsert, response)
    }

    /// Remove the motion of the entity if there is any
    pub fn stop_movement(
        &mut self,
        new_ref_time: f64,
        new_ref_center: Point,
    ) -> (bool, MovementResponse) {
        self.ref_center = new_ref_center;
        let old_movement = self.movement.take();

        // If movement was removed from this entity, then use that movement
        // to determine whether or not to reinsert the windowed pointer
        let needs_reinsert = if let Some(movement) = old_movement {
            let time_to_boundary = self.time_until_boundary_for(&movement);
            if time_to_boundary < MIN_SECS_BETWEEN_REINSERTS {
                self.recenter_window();
                true
            } else {
                false
            }
        } else {
            false
        };

        (
            needs_reinsert,
            MovementResponse::Stopped {
                center: new_ref_center,
            },
        )
    }

    /// Check to see if the entity plus radius contains the given point at the
    /// given time
    pub fn check_entity_contains(&self, point: &Point, time: f64) -> bool {
        self.get_center_at_time(time).distance_squared_to(point)
            <= self.radius * self.radius
    }
}

impl Movement {
    /// Get the movement of the entity's center at the given time
    fn position_offset_at(&self, time: f64) -> Point {
        let sample_time = time.min(self.max_valid_time);
        self.velocity * (sample_time - self.ref_time)
    }
}
