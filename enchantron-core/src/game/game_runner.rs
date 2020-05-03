use super::{GameEntity, WorldService};
use crate::event::*;
use crate::model::{IPoint, IRect};
use tokio::time::{delay_for, Duration};

#[derive(derive_new::new)]
pub struct GameRunner {
    event_bus: EventBus,
    world_service: WorldService,
}

impl GameRunner {
    pub async fn run(&self) {
        self.run_player().await;
    }

    async fn run_player(&self) {
        let world_key = self
            .world_service
            .insert(GameEntity::Player, IPoint::new(0, 0))
            .await;

        self.event_bus.spawn(async move {
            loop {
                delay_for(Duration::from_secs(1)).await;

                self.world_service
                    .move_by_key_delta(&world_key, &IPoint::new(0, -1))
                    .await
            }
        });
    }
}
