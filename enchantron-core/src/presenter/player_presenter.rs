use super::EntityPresenter;
use crate::game::location::{
    LocationService, MovementRequest, MovementResponse,
};
use crate::game::{
    EntityMessage, EntityRunBundle, Player, PresenterServiceLease, Time,
};
use crate::model::{IPoint, Point};
use crate::view::PlayerView;
use tokio::select;
use tokio::sync::mpsc::Receiver;

const TILE_CENTER_SHIFT: Point = Point { x: 0.5, y: 0.5 };

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

macro_rules! wait_until_interrupted {
    ($this:ident) => {
        if let Some(val) = $this.interrupts.recv().await {
            $this.handle_interrupt(val);
            continue;
        } else {
            break;
        }
    };
}

macro_rules! interruptible {
    ($this:ident$(.$prop_or_func:ident)+($($arg:expr),*)) => {
        handle_interrupts!($this, $this$(.$prop_or_func)+($($arg),*));
    };
}

fn move_response_to_state(resp: MovementResponse) -> CoarseState {
    match resp {
        MovementResponse::ArrivalPredicted { time } => {
            CoarseState::Arriving { arrival_time: time }
        }
        MovementResponse::MaintenanceNeeded { time } => CoarseState::Walking {
            next_update_time: time,
        },
        MovementResponse::Stopped { center } => CoarseState::Idle,
        _ => todo!(),
    }
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
    Spawning,
    Idle,
    StartWalk { start_time: f64, target: Point },
    Walking { next_update_time: f64 },
    Arriving { arrival_time: f64 },
}

impl Default for PlayerPresenterState {
    fn default() -> Self {
        PlayerPresenterState {
            coarse_state: CoarseState::Spawning,
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
                self.state.move_target = Some(target_tile);
                self.state.coarse_state = CoarseState::StartWalk {
                    start_time: self.time.now(),
                    target: target_tile.as_point() + &TILE_CENTER_SHIFT,
                };
            }
        }
    }

    pub async fn run(mut self) {
        info!("Player presenter spawned");

        loop {
            use CoarseState::*;

            match self.state.coarse_state {
                Spawning => {
                    self.view.as_ref().map(V::rest);

                    interruptible!(self.time.sleep_until(0.5));

                    self.state.coarse_state = Idle;
                }
                Idle => {
                    self.view.as_ref().map(V::rest);

                    wait_until_interrupted!(self);
                }
                StartWalk { start_time, target } => {
                    let resp = self
                        .location_service
                        .update_movement(
                            &self.player.location_key,
                            MovementRequest::MoveToward {
                                target: target,
                                speed: 1.0,
                            },
                        )
                        .await
                        .expect("Player should always be present");

                    self.state.coarse_state = move_response_to_state(resp);
                }
                Walking { next_update_time } => {
                    interruptible!(self.time.sleep_until(next_update_time));
                    let resp = self
                        .location_service
                        .update_movement(
                            &self.player.location_key,
                            MovementRequest::Maintain,
                        )
                        .await
                        .expect("Player should always be present");

                    self.state.coarse_state = move_response_to_state(resp);
                }
                Arriving { arrival_time } => {
                    interruptible!(self.time.sleep_until(arrival_time));
                    let resp = self
                        .location_service
                        .update_movement(
                            &self.player.location_key,
                            MovementRequest::Stop,
                        )
                        .await
                        .expect("Player should always be present");

                    self.state.coarse_state = move_response_to_state(resp);
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
