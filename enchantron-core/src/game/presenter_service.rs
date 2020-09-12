use super::{Entity, EntityRunBundle, EntityType, Services};
use crate::native::RuntimeResources;
use crate::presenter::*;
use crate::ui::SpriteSource;
use crate::view::*;
use crate::view_types::ViewTypes;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::sync::Arc;
use tokio::runtime::Handle;
use tokio::sync::RwLock;

/// This is how the presenter service shares the state of each presenter with
/// the actual presenters. The presenters each get a loan of the state that they
/// can mutate as if they owned it, but since the presenters only run when time
/// is not paused, the presenter service can read from the state whenever time
/// is paused. The presenter service could also modify the presenter's state,
/// while time is paused, but that doesn't seem smart
pub struct PresenterServiceLoan<T> {
    loaned: *const T,
}

unsafe impl<T> Send for PresenterServiceLoan<T> where T: Send + Sync {}
unsafe impl<T> Sync for PresenterServiceLoan<T> where T: Send + Sync {}

#[derive(Clone, Debug)]
pub struct PresenterService {
    inner: Arc<Inner>,
}

#[derive(Debug)]
struct Inner {
    player_presenter_states: RwLock<HashMap<Entity, Box<PlayerPresenterState>>>,
}

impl PresenterService {
    pub fn new(
        player_presenter_states: impl Iterator<
            Item = (Entity, PlayerPresenterState),
        >,
    ) -> PresenterService {
        PresenterService {
            inner: Arc::new(Inner {
                player_presenter_states: RwLock::new(HashMap::from_iter(
                    player_presenter_states.map(|(k, v)| (k, Box::new(v))),
                )),
            }),
        }
    }

    pub async fn get_player_presenter_state(
        &self,
        player_entity: Entity,
    ) -> Option<&PlayerPresenterState> {
        return self
            .player_entity
            .read()
            .expect("Unable to read player-presenter states")
            .get(&player_entity)
            .map(Box::as_ref);
    }
}
