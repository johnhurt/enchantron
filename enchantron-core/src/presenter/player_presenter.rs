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

macro_rules! wait_until_interrupted {
    ($this:ident) => {
        if let Some(val) = $this.interrupts.recv().await {
            $this.handle_interrupt(val);
            continue;
        }
        else {
            break;
        }
    }
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
    StarWalk(f64),
    Walking(f64, f64),
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
                self.state.move_target = Some(target_tile);
                self.state.coarse_state = CoarseState::StarWalk(self.time.now());
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

                    wait_until_interrupted!(self);
                }
                StarWalk(start) => {
                    let dist =
                    self.location_service.update_by_key(self.player.location_key, new_velocity_opt)
                }
                Walking(start, next_update) => {
                    interruptible!(self.time.sleep_until(start + 1.));
                    self.state.coarse_state = Walking(self.time.now());
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
