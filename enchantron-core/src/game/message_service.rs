use super::{Entity, EntityMessage, EntityType};
use crate::util::ConcurrentSlotmap;
use one_way_slot_map::SlotMap;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;

#[derive(Clone, Debug)]
pub struct MessageService {
    inner: Arc<Inner>,
}

#[derive(Debug)]
struct Inner {
    entity_messagers:
        ConcurrentSlotmap<Entity, EntityType, Box<Sender<EntityMessage>>>,
}

impl MessageService {
    pub fn new(
        messagers: SlotMap<Entity, EntityType, Box<Sender<EntityMessage>>>,
    ) -> MessageService {
        MessageService {
            inner: Arc::new(Inner {
                entity_messagers: ConcurrentSlotmap::new_with_data(messagers),
            }),
        }
    }
}
