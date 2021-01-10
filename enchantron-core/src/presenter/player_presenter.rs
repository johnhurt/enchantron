use super::EntityPresenter;
use crate::game::{
    Direction, EntityMessage, EntityRunBundle, LocationService, Player,
    PresenterServiceLease, Services, Time,
};
use crate::model::IPoint;
use crate::view::PlayerView;
use tokio::select;
use tokio::sync::mpsc::Receiver;

macro_rules! handle_interrupts {
    ($this:ident, $interruptible:expr) => {
        let response = select! {
            _ = $interruptible => None,
            message = $this.interrupts.recv() => Some(message)
        };

        match response {
            Some(Some(val)) => {
                $this.handle_interrupt(val);
                continue;
            }
            Some(None) => {
                break;
            }
            None => {}
        }
    };
}

macro_rules! interruptible {
    ($this:ident$(.$prop_or_func:ident)+($($arg:expr),*)) => {
        handle_interrupts!($this, $this$(.$prop_or_func)+($($arg),*));
    };
}

pub struct PlayerPresenter<F, V>
where
    F: Fn() -> V + 'static + Send,
    V: PlayerView,
{
    view: Option<V>,
    player: Player,
    view_provider: F,
    time: Time,
    location_service: LocationService,
    state: PresenterServiceLease<PlayerPresenterState>,
    interrupts: Receiver<EntityMessage>,
}

#[derive(Debug, Clone, Copy)]
pub struct PlayerPresenterState {
    coarse_state: CoarseState,
    move_target: Option<IPoint>,
}

#[derive(Debug, Clone, Copy)]
enum CoarseState {
    Spawning(f64),
    Idle(f64),
    WalkingOut(f64),
    WalkingIn(f64),
}

impl Default for PlayerPresenterState {
    fn default() -> Self {
        PlayerPresenterState {
            coarse_state: CoarseState::Spawning(0.),
            move_target: None,
        }
    }
}

impl<F, V> PlayerPresenter<F, V>
where
    F: Fn() -> V + 'static + Send,
    V: PlayerView,
{
    pub fn new(
        entity_bundle: EntityRunBundle,
        state: PresenterServiceLease<PlayerPresenterState>,
        view_provider: F,
    ) -> PlayerPresenter<F, V> {
        info!("Creating player presenter");

        let EntityRunBundle {
            entity: _,
            entity_data,
            entity_message_source: interrupts,
            services,
        } = entity_bundle;

        PlayerPresenter {
            view: Some(view_provider()),
            player: Player::from(&entity_data),
            view_provider,
            time: services.time(),
            location_service: services.location_service(),
            state,
            interrupts,
        }
    }

    fn handle_interrupt(&mut self, interrupt: EntityMessage) {
        match interrupt {
            EntityMessage::EnteredViewport => {
                self.view = Some((self.view_provider)())
            }
            EntityMessage::ExitedViewport => {
                drop(self.view.take());
            }
            EntityMessage::GoalSet(target_tile) => {
                debug!("Goal set {:?}", target_tile);
            }
        }
    }

    pub async fn run(mut self) {
        info!("Player presenter spawned");

        loop {
            use CoarseState::*;

            match self.state.coarse_state {
                Spawning(start) => {
                    self.view.as_ref().map(V::rest);

                    interruptible!(self.time.sleep_until(start + 0.5));

                    self.state.coarse_state = Idle(self.time.now());
                }
                Idle(start) => {
                    self.view.as_ref().map(V::rest);

                    interruptible!(self.time.sleep_until(start + 0.5));

                    self.state.coarse_state = WalkingIn(self.time.now());
                }
                WalkingIn(start) => {
                    let tile = self
                        .location_service
                        .get_by_key(&self.player.location_key)
                        .await
                        .unwrap()
                        .top_left;

                    self.view.as_ref().map(|view| {
                        view.start_walk(Direction::SOUTH, &tile, start, 0.5)
                    });

                    interruptible!(self.time.sleep_until(start + 1.));
                    self.state.coarse_state = WalkingOut(self.time.now());
                }
                WalkingOut(start) => {
                    let tile = self
                        .location_service
                        .get_by_key(&self.player.location_key)
                        .await
                        .unwrap()
                        .top_left;

                    self.location_service
                        .move_by_key_delta(
                            &self.player.location_key,
                            Direction::SOUTH.get_point(),
                        )
                        .await;

                    self.view.as_ref().map(|view| {
                        view.finish_walk(Direction::SOUTH, &tile, start, 0.5);
                    });
                    interruptible!(self.time.sleep_until(start + 1.));
                    self.state.coarse_state = Idle(self.time.now());
                }
            }
        }
    }
}

impl<F, V> EntityPresenter for PlayerPresenter<F, V>
where
    F: Fn() -> V + Send + 'static,
    V: PlayerView,
{
    type View = V;

    fn create_view(&self) -> Self::View {
        (self.view_provider)()
    }
}
