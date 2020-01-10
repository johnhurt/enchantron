
use itertools::Itertools;
use std::any::Any;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::pin::Pin;
use std::sync::{Arc, Weak};

use super::{Event, EventKey, EventListener, ListenerRegistration};

use crate::util::{BoxedAny, SimpleSlotMap};

use atomic_counter::{AtomicCounter, RelaxedCounter};

use async_chashmap::CHashMap;

use tokio::runtime::Handle;
use tokio::sync::mpsc::{
    unbounded_channel, UnboundedReceiver as Receiver, UnboundedSender as Sender,
};
use tokio::stream::StreamExt;

const WORKER_COUNT: usize = 16;

/// Centeralizable component for blindly passing and receiving messages.
/// Internally this contains an arc, so this object can be cloned without worry
#[derive(Clone)]
pub struct EventBus<K: EventKey> {
    inner: Arc<InnerEventBus<K>>,
}

/// Impl details for event bus
struct InnerEventBus<K: EventKey> {
    listeners: CHashMap<K,SimpleSlotMap<Sender<BoxedAny>>>,
    event_counter: RelaxedCounter,
    threadpool: Handle,
}

impl<K: EventKey> InnerEventBus<K> {
    fn create(
        threadpool: Handle,
    ) -> InnerEventBus<K>) {
        InnerEventBus {
            listeners: CHashMap::default(),
            event_counter: RelaxedCounter::new(0),
            threadpool,
        }

    }

}

impl<K: EventKey> EventBus<K> {
    pub fn new(tokio_runtime_handle: &Handle) -> EventBus<K> {
        let inner_event_bus = InnerEventBus::create(tokio_runtime_handle.clone());

        let inner_event_bus_arc = Arc::new(inner_event_bus);

        EventBus {
            inner: inner_event_bus_arc,
        }
    }

    /// Register the given event listener to listen for events of the given type, and
    /// return a registration that can be used to deregister the listener from this
    /// event type
    pub async fn register<E: Event<K>>(
        &self,
        event_key: K,
    ) -> (ListenerRegistration, Receiver<E>) {
        let inner_clone = self.inner.clone();

        let mut slot_map_key_opt: Option<usize> = None;

        {
            let slot_map_key_opt_ref = &mut slot_map_key_opt;

            info!("Adding a listener for {:?}", &event_key);

            let (sender, receiver) : (Sender<BoxedAny>, Receiver<BoxedAny>)
                = unbounded_channel();

            inner_clone.listeners.alter(event_key, move | listeners_opt | {
                let mut listeners = match listeners_opt {
                    None => SimpleSlotMap::new(),
                    Some(existing_listeners) => existing_listeners
                };

                *slot_map_key_opt_ref = Some(listeners.insert(sender));

                info!("Got key {:?}", *slot_map_key_opt_ref);
                info!("There are now {} listeners for {:?} Events", listeners.len(), &event_key);

                Some(listeners)
            }).await;
        }

        info!("Now the key is {:?}", slot_map_key_opt);

        let slot_map_key = slot_map_key_opt.unwrap_or_else(|| {
            error!("Failed to set slot map key when adding event listener");
            panic!("Failed to set slot map key when adding event listener");
        });

        // Create and return the registration
        ( ListenerRegistration::new(Box::new(move || {
            info!("Deregistering listener for event {:?}", &event_key);

            let another_clone = inner_clone.clone();

            inner_clone.threadpool.spawn(async move {

                another_clone
                    .listeners
                    .alter(event_key, |listeners_opt| match listeners_opt {
                        None => {
                            warn!(
                                "Attempted to remove a listener from an empty map"
                            );
                            None
                        }
                        Some(mut listeners) => {
                            let _ = listeners.remove(slot_map_key);
                            Some(listeners)
                        }
                    })
                    .await
            });
        })), )
    }

    /// Post the given event to the event bus.  This event will be distributed
    /// to all listeners registered to accept the given event
    pub fn post<E: Event<K>>(&self, event: E) {
        self.post_with_partition(event, self.inner.event_counter.inc());
    }

    pub fn post_with_partition<E: Event<K>, P: Hash>(
        &self,
        event: E,
        partition_key: P,
    ) {
        let mut h: DefaultHasher = Default::default();
        partition_key.hash(&mut h);

        let sink_index = (h.finish() as usize) % WORKER_COUNT;

        debug!("Posting event {:?} on sink {}", event, sink_index);

        match self.inner.sinks[sink_index]
            .send((event.get_event_key(), Box::new(event)))
        {
            Err(err) => {
                error!("Failed to publish event to channel: {:?}", err);
                panic!("Failed to publish event to channel");
            }
            _ => {}
        };
    }
}
