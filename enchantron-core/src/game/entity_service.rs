use super::{Entity, EntityData, EntityType};
use crate::util::ConcurrentSlotmap;
use one_way_slot_map::SlotMap;
use std::sync::Arc;

/// This is effectively the main service. It controls entity creation, storage,
/// and messaging.
#[derive(Clone, Debug)]
pub struct EntityService {
    inner: Arc<Inner>,
}

impl EntityService {
    pub fn new_with_data(
        entities_data: SlotMap<Entity, EntityType, EntityData>,
    ) -> EntityService {
        let entities = ConcurrentSlotmap::new_with_data(entities_data);

        EntityService {
            inner: Arc::new(Inner::new(entities)),
        }
    }

    // pub async fn initialize<F>(&self) -> impl Iter<Item = EntityRunBundle> {
    //     let mut pre_init_data = self
    //         .inner
    //         .pre_init_data
    //         .lock()
    //         .await
    //         .take()
    //         .expect("Only init once, Bro");

    //     let this = self.clone();
    //     let this_provider = || this.clone();

    //     let reader = BoxRef::new(Box::new(self.inner.entities.reader()));

    //     reader.map(|reader_ref| {
    //         &reader_ref
    //             .iter(|e| e.entity_type)
    //             .zip(pre_init_data.values_mut())
    //             .map(move |((k, e), channel_opt)| {
    //                 let this = this_provider();
    //                 let time = this.inner.time.clone();

    //                 let (_, receiver) = channel_opt
    //                     .take()
    //                     .expect("Channel was just added and can't be removed");
    //                 EntityRunBundle::new(k, *e, receiver, this, time)
    //             })
    //     })
    // }
}

#[derive(derive_new::new, Debug)]
struct Inner {
    entities: ConcurrentSlotmap<Entity, EntityType, EntityData>,
}
