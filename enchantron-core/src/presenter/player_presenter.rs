use crate::event::*;
use crate::game::{
    Direction, GameEntity, GameEntitySlotKey, PerlinTerrain1, Player,
    TerrainProvider, Time, WorldService,
};
use crate::model::IPoint;
use crate::view::PlayerView;
use std::marker::PhantomData;
use std::sync::Arc;

pub struct PlayerPresenter<V: PlayerView> {
    event_bus: EventBus,
    world_service: WorldService,
    time: Time,
    phantom_view: PhantomData<V>,
}

impl<V: PlayerView> PlayerPresenter<V> {
    pub fn new(
        event_bus: EventBus,
        world_service: WorldService,
        time: Time,
    ) -> Arc<PlayerPresenter<V>> {
        Arc::new(PlayerPresenter {
            event_bus,
            world_service,
            time,
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

    pub fn run(
        this: Arc<PlayerPresenter<V>>,
        player: Player,
        view_provider: impl Fn() -> V + 'static + Send,
    ) {
        info!("About to spawn player");

        let _ = this.event_bus.clone().spawn(async move {
            info!("Player presenter spawned");

            let terrain_generator = PerlinTerrain1::default();

            let view: V = view_provider();

            loop {
                view.rest();

                let start_tile = &this
                    .world_service
                    .get_by_key(&player.location_key)
                    .await
                    .unwrap()
                    .top_left;

                info!("resing in {:?}", terrain_generator.get_for(start_tile));

                this.time.sleep(0.5).await;

                info!("walking");

                view.start_walk(
                    Direction::SOUTH,
                    &start_tile,
                    this.time.now(),
                    0.5,
                );

                this.time.sleep(1.0).await;

                this.world_service
                    .move_by_key_delta(
                        &player.location_key,
                        Direction::SOUTH.get_point(),
                    )
                    .await;

                view.finish_walk(
                    Direction::SOUTH,
                    &start_tile,
                    this.time.now(),
                    0.5,
                );

                this.time.sleep(1.).await;
            }
        });

        info!("Finished spawning player");
    }
}
