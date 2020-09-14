use super::Entity;
use crate::presenter::*;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use tokio::sync::RwLock;

/// This is how the presenter service shares the state of each presenter with
/// the actual presenters. The presenters each get a loan of the state that they
/// can mutate as if they owned it, but since the presenters only run when time
/// is not paused, the presenter service can read from the state whenever time
/// is paused. The presenter service could also modify the presenter's state,
/// while time is paused, but that doesn't seem smart
pub struct PresenterServiceLease<T> {
    loaned: *const T,
}

/// SAFETY: This type is only shared once to the presenter it belongs to.
/// The presenter and the presenter service are forced through pausable-tokio
/// time to not operate on the data at the same time
unsafe impl<T> Send for PresenterServiceLease<T> where T: Send + Sync {}
unsafe impl<T> Sync for PresenterServiceLease<T> where T: Send + Sync {}

impl<T> PresenterServiceLease<T> {
    fn new(original: &T) -> PresenterServiceLease<T> {
        PresenterServiceLease { loaned: original }
    }
}

impl<T> Deref for PresenterServiceLease<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: This method is only called by the presenter and the
        // presenter only operates when time is not paused
        unsafe { &*self.loaned }
    }
}

impl<T> DerefMut for PresenterServiceLease<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: This method is only called by the presenter and the
        // presenter only operates when time is not paused
        unsafe { &mut *(self.loaned as *mut T) }
    }
}

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
    ) -> Option<PlayerPresenterState> {
        self.inner
            .player_presenter_states
            .read()
            .await
            .get(&player_entity)
            .map(Box::as_ref)
            .map(|state| *state)
    }

    pub async fn rent_player_presenter_state(
        &self,
        player_entity: &Entity,
    ) -> Option<PresenterServiceLease<PlayerPresenterState>> {
        self.inner
            .player_presenter_states
            .read()
            .await
            .get(player_entity)
            .map(Box::as_ref)
            .map(PresenterServiceLease::new)
    }
}
