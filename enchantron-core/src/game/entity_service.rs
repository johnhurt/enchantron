use super::{Entity, EntityData, EntityType, Gor, Player};
use crate::util::ConcurrentSlotmap;
use one_way_slot_map::SlotMap;

/// This is effectively the main service. It controls entity creation, storage,
/// and messaging.
#[derive(Clone, Debug)]
pub struct EntityService {
    inner: Gor<Inner>,
}

impl EntityService {
    pub fn new_with_data(
        entities_data: SlotMap<Entity, EntityType, EntityData>,
    ) -> (EntityService, impl FnOnce()) {
        let entities = ConcurrentSlotmap::new_with_data(entities_data);

        let boxed_inner = Box::new(Inner::new(entities));
        let inner = Gor::new(&boxed_inner);
        let dropper = move || drop(boxed_inner);

        (EntityService { inner }, dropper)
    }

    pub fn get_player(&self) -> Player {
        self.inner.entities.get(&Entity::Player(EntityType::Player)).as_ref().unwrap().into()
    }

}

#[derive(derive_new::new, Debug)]
struct Inner {
    entities: ConcurrentSlotmap<Entity, EntityType, EntityData>,
}
