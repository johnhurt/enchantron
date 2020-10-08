use super::Gor;
use super::{Entity, EntityMessage, EntityType};
use crate::util::ConcurrentSlotmap;
use one_way_slot_map::SlotMap;
use tokio::sync::mpsc::Sender;

#[derive(Clone, Debug)]
pub struct MessageService {
    inner: Gor<Inner>,
}

#[derive(Debug)]
struct Inner {
    entity_messagers:
        ConcurrentSlotmap<Entity, EntityType, Box<Sender<EntityMessage>>>,
}

impl MessageService {
    pub fn new(
        messagers: SlotMap<Entity, EntityType, Box<Sender<EntityMessage>>>,
    ) -> (MessageService, impl FnOnce()) {
        let boxed_inner = Box::new(Inner {
            entity_messagers: ConcurrentSlotmap::new_with_data(messagers),
        });

        let inner = Gor::new(&boxed_inner);

        (MessageService { inner }, move || drop(boxed_inner))
    }
}
