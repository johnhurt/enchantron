use super::{GameEntity, GameEntitySlotKey, WorldService};
use crate::event::*;
use crate::model::{IPoint, IRect};
use std::sync::Arc;
use tokio::time::{delay_for, Duration};

pub struct GameRunner {
    event_bus: EventBus,
    world_service: WorldService,
}

impl GameRunner {
    pub fn new(event_bus: EventBus) -> GameRunner {
        GameRunner {
            event_bus,
            world_service: WorldService::new(),
        }
    }

    pub async fn run(this: Arc<GameRunner>) {
        let eb = this.event_bus.clone();
        let world_key = GameRunner::initialize_player(&this).await;
        eb.spawn(GameRunner::run_player(this, world_key));
    }

    async fn initialize_player(this: &Arc<GameRunner>) -> GameEntitySlotKey {
        this.clone()
            .world_service
            .insert(GameEntity::Player, IPoint::new(0, 0))
            .await
    }

    async fn run_player(this: Arc<GameRunner>, world_key: GameEntitySlotKey) {}
}
