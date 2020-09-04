use crate::application_context::NUM_CPUS;
use crate::util::ImmutableThreadLocal;
use ev_slotmap::{MapReadRef, ReadGuard, ReadHandle, WriteHandle};
use evmap::ShallowCopy;
use one_way_slot_map::{SlotMap, SlotMapKey};
use tokio::sync::Mutex;

/// Centralization of the components of a ev_slotmap in a component that can
/// be shared safely across a _limited_ number of threads
#[derive(Debug)]
pub struct ConcurrentSlotmap<K, P, V>
where
    K: SlotMapKey<P> + Send,
    P: Send,
    V: ShallowCopy + Send,
{
    writer: Mutex<WriteHandle<K, P, V>>,
    readers: ImmutableThreadLocal<ReadHandle<K, P, V>>,
}

impl<K, P, V> ConcurrentSlotmap<K, P, V>
where
    K: SlotMapKey<P> + Send,
    P: Send,
    V: ShallowCopy + Send,
{
    pub fn new() -> ConcurrentSlotmap<K, P, V> {
        Self::new_with_data(SlotMap::new())
    }

    pub fn new_with_data(data: SlotMap<K, P, V>) -> ConcurrentSlotmap<K, P, V> {
        let (_, slotmap) = ev_slotmap::new_with_data(data);

        let num_readers = *NUM_CPUS * 3;
        let mut readers = Vec::with_capacity(num_readers);

        let reader_factory = slotmap.factory();

        for _ in 0..num_readers {
            readers.push(reader_factory.handle());
        }

        ConcurrentSlotmap {
            writer: Mutex::new(slotmap),
            readers: ImmutableThreadLocal::new(readers),
        }
    }

    pub async fn insert(&self, pointer: P, value: V) -> K {
        self.writer.lock().await.insert(pointer, value)
    }

    pub fn get(&self, key: &K) -> Option<ReadGuard<V>> {
        self.readers.get().get(key)
    }

    pub fn reader(&self) -> MapReadRef<'_, K, P, V> {
        self.readers
            .get()
            .read()
            .expect("Should always be readable")
    }
}
