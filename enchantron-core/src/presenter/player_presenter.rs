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

#[derive(Debug, Clone, Copy)]
pub enum PlayerPresenterState {
    Idle(f64),
    WalkingOut(f64),
    WalkingIn(f64),
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
        let mut state = PlayerPresenterState::Idle(time.now());

        loop {
            match state {
                PlayerPresenterState::Idle(start) => {
                    view.rest();
                    time.sleep_until(start + 0.5).await;
                    state = PlayerPresenterState::WalkingIn(time.now());
                }
                PlayerPresenterState::WalkingIn(start) => {
                    let tile = location_service
                        .get_by_key(&player.location_key)
                        .await
                        .unwrap()
                        .top_left;
                    view.start_walk(Direction::SOUTH, &tile, start, 0.5);
                    time.sleep_until(start + 1.0).await;
                    state = PlayerPresenterState::WalkingOut(time.now());
                }
                PlayerPresenterState::WalkingOut(start) => {
                    let tile = location_service
                        .get_by_key(&player.location_key)
                        .await
                        .unwrap()
                        .top_left;

                    location_service
                        .move_by_key_delta(
                            &player.location_key,
                            Direction::SOUTH.get_point(),
                        )
                        .await;

                    view.finish_walk(Direction::SOUTH, &tile, start, 0.5);
                    time.sleep_until(start + 1.0).await;
                    state = PlayerPresenterState::Idle(time.now());
                }
            }
        }
    }
}
