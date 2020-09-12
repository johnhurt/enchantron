use super::{Entity, EntityType, EntityRunBundle};
use crate::presenter::*;
use crate::view::*;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::runtime::Handle;
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

    pub fn run(runtime_handle: Handle, run_bundles: impl Iterator<Item = EntityRunBundle>) {

        info!("Initializing Entities");

        for run_bundle in run_bundles {
            let entity_sprite_group = entity_sprite_group.clone();
            let runtime_resources = runtime_resources.clone();
            let time = time.clone();

            match &run_bundle.entity_data.entity_type {
                EntityType::Player => {
                    let player_view_provider = move || {
                        info!("Providing new player sprite");

                        PlayerViewImpl::<T>::new(
                            entity_sprite_group.create_sprite(),
                            runtime_resources.clone(),
                            time.clone(),
                        )
                    };

                    runtime_handle.spawn(async move {
                        PlayerPresenter::run(run_bundle, player_view_provider)
                            .await
                    });
                }
            }
        }

    }
}
