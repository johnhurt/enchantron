use crate::event::*;
use crate::game::{
    Direction, Entity, EntityRunBundle, EntityService, LocationService,
    PerlinTerrain1, Player, TerrainProvider, Time,
};
use crate::model::IPoint;
use crate::view::PlayerView;
use std::marker::PhantomData;
use std::sync::Arc;

pub struct PlayerPresenter<V: PlayerView> {
    _phantom_view: PhantomData<V>,
}

impl<V: PlayerView> PlayerPresenter<V> {
    pub async fn run(
        entity_bundle: EntityRunBundle,
        view_provider: impl Fn() -> V + 'static + Send,
    ) {
        info!("Player presenter spawned");

        let EntityRunBundle {
            entity,
            entity_data,
            entity_message_source: mut recv,
            services,
        } = entity_bundle;

        let player = Player::from(&entity_data);
        let terrain_generator = PerlinTerrain1::default();
        let location_service = services.location_service();
        let time = services.time();

        let view: V = view_provider();

        loop {
            view.rest();

            let start_tile = location_service
                .get_by_key(&player.location_key)
                .await
                .unwrap()
                .top_left;

            info!("resting in {:?}", terrain_generator.get_for(&start_tile));

            time.sleep(0.5).await;

            info!("walking");

            view.start_walk(Direction::SOUTH, &start_tile, time.now(), 0.5);

            time.sleep(1.0).await;

            location_service
                .move_by_key_delta(
                    &player.location_key,
                    Direction::SOUTH.get_point(),
                )
                .await;

            view.finish_walk(Direction::SOUTH, &start_tile, time.now(), 0.5);

            time.sleep(1.).await;
        }
    }
}
