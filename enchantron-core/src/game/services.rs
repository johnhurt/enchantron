use super::{
    Entity, EntityData, EntityMessage, EntityRunBundle, EntityService,
    EntityType, LocationService, MessageService, PresenterService, SavedGame,
    Time,
};
use crate::model::IPoint;
use crate::native::RuntimeResources;
use crate::presenter::*;
use crate::ui::SpriteSource;
use crate::view::*;
use crate::view_types::ViewTypes;
use one_way_slot_map::SlotMap;
use std::sync::Arc;
use tokio::runtime::Handle;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::Mutex;

const ENTITY_MESSAGE_CHANNEL_SIZE: usize = 8;

#[derive(Debug)]
struct TemporaryChannel {
    entity: EntityData,
    sender: Sender<EntityMessage>,
    receiver: Option<Receiver<EntityMessage>>,
}

impl TemporaryChannel {
    fn to_run_bundle(&mut self, services: &Services) -> EntityRunBundle {
        EntityRunBundle::new(
            self.entity.entity.unwrap(),
            self.entity,
            self.receiver.take().unwrap(),
            services.clone(),
        )
    }
}

#[derive(Clone, Debug)]
pub struct Services {
    time: Time,
    location_service: LocationService,
    entity_service: EntityService,
    message_service: MessageService,
    presenter_service: PresenterService,
}

impl Services {
    pub fn new(
        runtime_handle: Handle,
        saved_game: SavedGame,
    ) -> (Services, Vec<EntityRunBundle>) {
        let SavedGame {
            seed,
            elapsed_millis,
            entities,
            locations,
            player_presenter_states,
        } = saved_game;

        let time = Time::new(runtime_handle.clone());
        let location_service = LocationService::new_from_data(&locations);

        let mut entity_channels = entities.map(|data| {
            let (send, recv) = channel(ENTITY_MESSAGE_CHANNEL_SIZE);
            TemporaryChannel {
                entity: *data,
                sender: send,
                receiver: Some(recv),
            }
        });

        let entity_service = EntityService::new_with_data(entities);

        let message_service = MessageService::new(
            entity_channels
                .map(|tmp_channel| Box::new(tmp_channel.sender.clone())),
        );

        let presenter_service =
            PresenterService::new(player_presenter_states.into_iter());

        let services = Services {
            time,
            location_service,
            entity_service,
            message_service,
            presenter_service,
        };

        let run_bundles = entity_channels
            .drain()
            .map(|tmp_channel| tmp_channel.to_run_bundle(&services))
            .collect();

        (services, run_bundles)
    }

    pub async fn run<T: ViewTypes>(
        &self,
        runtime_handle: Handle,
        entity_sprite_group: Arc<T::SpriteGroup>,
        runtime_resources: Arc<RuntimeResources<T>>,
        run_bundles: impl Iterator<Item = EntityRunBundle>,
    ) {
        info!("Initializing Entities");

        let presenter_service = self.presenter_service();

        for run_bundle in run_bundles {
            let entity_sprite_group = entity_sprite_group.clone();
            let runtime_resources = runtime_resources.clone();
            let time = self.time();

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

                    let player_presenter_state = presenter_service
                        .rent_player_presenter_state(&run_bundle.entity)
                        .await
                        .expect("missing player presenter state");

                    runtime_handle.spawn(async move {
                        PlayerPresenter::run(
                            run_bundle,
                            player_presenter_state,
                            player_view_provider,
                        )
                        .await
                    });
                }
            }
        }
    }

    pub fn location_service(&self) -> LocationService {
        self.location_service.clone()
    }

    pub fn time(&self) -> Time {
        self.time.clone()
    }

    pub fn presenter_service(&self) -> PresenterService {
        self.presenter_service.clone()
    }
}
