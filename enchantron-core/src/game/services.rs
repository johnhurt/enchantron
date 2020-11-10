use super::{
    EntityData, EntityMessage, EntityRunBundle, EntityService, EntityType, Gor,
    LocationService, MessageService, PresenterService, SavedGame, Time,
};
use crate::application_context::Ao;
use crate::native::RuntimeResources;
use crate::presenter::*;
use crate::ui::SpriteSource;
use crate::view::*;
use crate::view_types::ViewTypes;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{channel, Receiver, Sender};

const ENTITY_MESSAGE_CHANNEL_SIZE: usize = 8;

#[derive(Debug)]
struct TemporaryChannel {
    entity: EntityData,
    sender: Sender<EntityMessage>,
    receiver: Option<Receiver<EntityMessage>>,
}

impl TemporaryChannel {
    fn as_run_bundle(&mut self, services: &Services) -> EntityRunBundle {
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
    runtime: Gor<Runtime>,

    time: Time,
    location_service: LocationService,
    entity_service: EntityService,
    message_service: MessageService,
    presenter_service: PresenterService,
}

impl Services {
    pub fn new(
        boxed_runtime: Box<Runtime>,
        saved_game: SavedGame,
    ) -> (
        Services,
        Vec<EntityRunBundle>,
        Vec<Box<dyn FnOnce() + Send>>,
    ) {
        let SavedGame {
            seed,
            elapsed_millis,
            entities,
            locations,
            player_presenter_states,
            player,
        } = saved_game;

        let runtime = Gor::new(&boxed_runtime);
        let runtime_dropper = move || drop(boxed_runtime);
        let time = Time::new(runtime.clone());
        let (location_service, location_service_dropper) =
            LocationService::new_from_data(&locations);

        let mut entity_channels = entities.map(|data| {
            let (send, recv) = channel(ENTITY_MESSAGE_CHANNEL_SIZE);
            TemporaryChannel {
                entity: *data,
                sender: send,
                receiver: Some(recv),
            }
        });

        let (entity_service, entity_service_dropper) =
            EntityService::new_with_data(player, entities);

        let (message_service, message_service_dropper) = MessageService::new(
            entity_channels
                .map(|tmp_channel| Box::new(tmp_channel.sender.clone())),
        );

        let (presenter_service, presenter_service_dropper) =
            PresenterService::new(player_presenter_states.into_iter());

        let services = Services {
            runtime,
            time,
            location_service,
            entity_service,
            message_service,
            presenter_service,
        };

        let run_bundles = entity_channels
            .drain()
            .map(|tmp_channel| tmp_channel.as_run_bundle(&services))
            .collect();

        let droppers: Vec<Box<dyn FnOnce() + Send>> = vec![
            Box::new(runtime_dropper),
            Box::new(location_service_dropper),
            Box::new(entity_service_dropper),
            Box::new(message_service_dropper),
            Box::new(presenter_service_dropper),
        ];

        (services, run_bundles, droppers)
    }

    pub async fn run<T: ViewTypes>(
        &self,
        entity_sprite_group: Gor<T::SpriteGroup>,
        runtime_resources: Ao<RuntimeResources<T>>,
        run_bundles: impl Iterator<Item = EntityRunBundle>,
    ) {
        info!("Initializing Entities");

        let presenter_service = self.presenter_service();
        let runtime = self.runtime();

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

                    let presenter = PlayerPresenter::new(
                        run_bundle,
                        player_presenter_state,
                        player_view_provider,
                    );

                    runtime.spawn(presenter.run());
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

    pub fn runtime(&self) -> Gor<Runtime> {
        self.runtime.clone()
    }

    pub fn entity_service(&self) -> EntityService {
        self.entity_service.clone()
    }

    pub fn message_service(&self) -> MessageService {
        self.message_service.clone()
    }
}
