use crate::event::*;
use crate::game::{
    Direction, Entity, EntityService, LocationService, PerlinTerrain1, Player,
    TerrainProvider, Time,
};
use crate::model::IPoint;
use crate::view::PlayerView;
use std::marker::PhantomData;
use std::sync::Arc;

pub struct PlayerPresenter<V: PlayerView> {
    event_bus: EventBus,
    location_service: LocationService,
    time: Time,
    player: Player,

    _phantom_view: PhantomData<V>,
}

impl<V: PlayerView> PlayerPresenter<V> {
    pub fn new(
        event_bus: EventBus,
        entity_service: EntityService,
        time: Time,
    ) -> Arc<PlayerPresenter<V>> {
        Arc::new(PlayerPresenter {
            event_bus,
            location_service: entity_service.location_service(),
            time,
            _phantom_view: Default::default(),
        })
    }

    pub fn run(
        this: Arc<PlayerPresenter<V>>,
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
                    .location_service
                    .get_by_key(&this.player.location_key)
                    .await
                    .unwrap()
                    .top_left;

                info!("resting in {:?}", terrain_generator.get_for(start_tile));

                this.time.sleep(0.5).await;

                info!("walking");

                view.start_walk(
                    Direction::SOUTH,
                    &start_tile,
                    this.time.now(),
                    0.5,
                );

                this.time.sleep(1.0).await;

                this.location_service
                    .move_by_key_delta(
                        &this.player.location_key,
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
