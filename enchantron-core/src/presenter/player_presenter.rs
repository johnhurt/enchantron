use crate::event::*;
use crate::game::{
    Direction, GameEntity, GameEntitySlotKey, Player, WorldService,
};
use crate::model::IPoint;
use crate::view::PlayerView;
use std::marker::PhantomData;
use std::sync::Arc;
use tokio::time::{delay_for, Duration};

pub struct PlayerPresenter<V: PlayerView> {
    event_bus: EventBus,
    world_service: WorldService,

    phantom_view: PhantomData<V>,
}

impl<V: PlayerView> PlayerPresenter<V> {
    pub fn new(
        event_bus: EventBus,
        world_service: WorldService,
    ) -> Arc<PlayerPresenter<V>> {
        Arc::new(PlayerPresenter {
            event_bus,
            world_service,
            phantom_view: Default::default(),
        })
    }

    pub async fn init(&self) -> Player {
        let location_key = self
            .world_service
            .insert(GameEntity::Player, IPoint::new(0, 0))
            .await;
        Player::new(location_key)
    }

    pub async fn run(
        this: Arc<PlayerPresenter<V>>,
        player: Player,
        view_provider: impl Fn() -> V + 'static + Send,
    ) {
        this.event_bus.clone().spawn(async move {
            let mut view: Option<V> = None;

            loop {
                tokio::select! {}

                view.start_walk(Direction::NORTH, 0.5);

                delay_for(Duration::from_secs(1)).await;

                this.world_service
                    .move_by_key_delta(
                        &player.location_key,
                        &IPoint::new(0, -1),
                    )
                    .await;

                view.finish_walk(Direction::NORTH, 0.5);

                delay_for(Duration::from_secs(1)).await;
            }
        });
    }
}
