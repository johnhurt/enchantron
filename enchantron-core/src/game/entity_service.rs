use super::{
    Entity, EntityData, EntityMessage, EntityType, LocationKey, LocationService,
};
use crate::event::*;
use crate::model::IPoint;
use crate::util::ConcurrentSlotmap;
use one_way_slot_map::SlotMap;
use std::sync::Arc;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::Mutex;

const ENTITY_MESSAGE_CHANNEL_SIZE: usize = 8;

/// This is effectively the main service. It controls entity creation, storage,
/// and messaging.
#[derive(Clone)]
pub struct EntityService {
    inner: Arc<Inner>,
}

impl EntityService {
    pub async fn new(event_bus: EventBus) -> EntityService {
        let entities_data = SlotMap::new();

        let mut player_data = EntityData::default_for_type(EntityType::Player);
        let player_key = entities_data.insert(EntityType::Player, player_data);

        let location_service = LocationService::new();

        let location_key =
            location_service.insert(player_key, IPoint::default()).await;

        player_data.location_key = Some(location_key);

        {
            let to_replace = entities_data
                .get_mut(&player_key)
                .expect("Player just added");
            *to_replace = player_data;
        }

        let entity_channels = entities_data.map(|_| {
            Some(channel::<EntityMessage>(ENTITY_MESSAGE_CHANNEL_SIZE))
        });
        let entity_messagers = ConcurrentSlotmap::new_with_data(
            entity_channels.map(|channel_opt| {
                Box::new(channel_opt.expect("Channels just added").0.clone())
            }),
        );

        let entities = ConcurrentSlotmap::new_with_data(entities_data);

        let inner = Inner::new(
            event_bus,
            LocationService::new(),
            entities,
            entity_messagers,
            Mutex::new(Some(entity_channels)),
        );

        EntityService {
            inner: Arc::new(inner),
        }
    }

    pub fn location_service(&self) -> LocationService {
        self.inner.location_service.clone()
    }

    pub async fn initialize(
        &self,
    ) -> impl Iterator<Item = (Entity, &EntityData, Receiver<EntityMessage>)>
    {
        let pre_init_data = self
            .inner
            .pre_init_data
            .lock()
            .await
            .take()
            .expect("Only init once, Bro");

        self.inner
            .entities
            .iter(|e| e.entity_type)
            .zip(pre_init_data.values_mut())
            .map(|((k, e), channel_opt)| {
                if let Some((_, receiver)) = channel_opt.take() {
                    (k, e, receiver)
                } else {
                    unreachable!("Channel was just added and can't be removed");
                }
            })
    }
}

#[derive(derive_new::new)]
struct Inner {
    event_bus: EventBus,
    location_service: LocationService,
    entities: ConcurrentSlotmap<Entity, EntityType, EntityData>,
    entity_messagers:
        ConcurrentSlotmap<Entity, EntityType, Box<Sender<EntityMessage>>>,
    pre_init_data: Mutex<
        Option<
            SlotMap<
                Entity,
                EntityType,
                Option<(Sender<EntityMessage>, Receiver<EntityMessage>)>,
            >,
        >,
    >,
}
