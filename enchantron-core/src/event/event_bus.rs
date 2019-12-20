use std::any::Any;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Weak};

use super::{Event, EventKey, EventListener, ListenerRegistration};

use crate::util::SimpleSlotMap;

use atomic_counter::{AtomicCounter, RelaxedCounter};

use chashmap::CHashMap;

use tokio::runtime::Handle;
use tokio::sync::mpsc::{
    unbounded_channel, UnboundedReceiver as Receiver, UnboundedSender as Sender,
};

const WORKER_COUNT: usize = 16;

type BoxedAny = Box<dyn Any + Send + Sync + 'static>;

/// Centeralizable component for blindly passing and receiving messages.
/// Internally this contains an arc, so this object can be cloned without worry
#[derive(Clone)]
pub struct EventBus<K: EventKey> {
    inner: Arc<InnerEventBus<K>>,
}

/// Impl details for event bus
struct InnerEventBus<K: EventKey> {
    listeners: CHashMap<K, SimpleSlotMap<Box<dyn Fn(&dyn Any) + Send + Sync>>>,
    sinks: Vec<Sender<(K, BoxedAny)>>,
    event_counter: RelaxedCounter,
}

struct EventBusEvaluator<K: EventKey> {
    inner_event_bus: Arc<InnerEventBus<K>>,
    receiver: Receiver<(K, BoxedAny)>,
}

impl<K: EventKey> InnerEventBus<K> {
    fn create() -> (InnerEventBus<K>, Vec<Receiver<(K, BoxedAny)>>) {
        let mut sinks: Vec<Sender<(K, BoxedAny)>> = Vec::new();
        let mut sources: Vec<Receiver<(K, BoxedAny)>> = Vec::new();

        for _ in 0..WORKER_COUNT {
            let (sink, source): (
                Sender<(K, BoxedAny)>,
                Receiver<(K, BoxedAny)>,
            ) = unbounded_channel();
            sinks.push(sink);
            sources.push(source);
        }

        (
            InnerEventBus {
                listeners: CHashMap::default(),
                sinks: sinks,
                event_counter: RelaxedCounter::new(0),
            },
            sources,
        )
    }

    async fn run_evaluator(
        self: Arc<InnerEventBus<K>>,
        mut receiver: Receiver<(K, BoxedAny)>,
    ) {
        while let Some((key, arg)) = receiver.recv().await {
            debug!("Firing {:?} - {:?}", key, arg);

            if let Some(handlers) = self.listeners.get(&key) {
                handlers.iter().for_each(|func| func(&*arg)); // <- Note the deref before borrow
            } else {
                info!("No handlers found for event key: {:?}", key);
            }

            debug!("Fired {:?}", key);
        }
    }
}

impl<K: EventKey> EventBus<K> {
    pub fn new(tokio_runtime_handle: &Handle) -> EventBus<K> {
        let (inner_event_bus, sources): (
            InnerEventBus<K>,
            Vec<Receiver<(K, BoxedAny)>>,
        ) = InnerEventBus::create();

        let inner_event_bus_arc = Arc::new(inner_event_bus);

        sources.into_iter().enumerate().for_each(|(i, source)| {
            let local_inner_event_bus_arc = inner_event_bus_arc.clone();
            tokio_runtime_handle.spawn(InnerEventBus::run_evaluator(
                local_inner_event_bus_arc,
                source,
            ));
        });

        EventBus {
            inner: inner_event_bus_arc,
        }
    }

    /// Register the given event listener to listen for events of the given type, and
    /// return a registration that can be used to deregister the listener from this
    /// event type
    pub fn register<E: Event<K>, H: EventListener<K, E>>(
        &self,
        event: E,
        listener: Weak<H>,
    ) {
        let listener_for_registration = {
            if let Some(listener_arc) = listener.upgrade() {
                listener_arc
            } else {
                return;
            }
        };

        let inner_clone = self.inner.clone();

        let event_key = event.get_event_key();
        let mut slot_map_key_opt: Option<usize> = None;

        {
            let slot_map_key_opt_ref = &mut slot_map_key_opt;

            info!("Adding a listener for {:?}", &event_key);

            inner_clone.listeners.alter(event_key, move | listeners_opt | {
                let mut listeners = match listeners_opt {
                    None => SimpleSlotMap::new(),
                    Some(existing_listeners) => existing_listeners
                };

                *slot_map_key_opt_ref = Some(listeners.insert( Box::new(move |arg| {
                    if let Some(handler) = listener.upgrade() {

                        if let Some(arg) = arg.downcast_ref::<E>() {
                            handler.on_event(arg);
                        }
                        else {
                            error!("Unable to downcast any ref to correct event type");
                            panic!("Unable to downcast any ref to correct event type");
                        };
                    }
                })));

                info!("Got key {:?}", *slot_map_key_opt_ref);
                info!("There are now {} listeners for {:?} Events", listeners.len(), &event_key);

                Some(listeners)
            });
        }

        info!("Now the key is {:?}", slot_map_key_opt);

        let slot_map_key = slot_map_key_opt.unwrap_or_else(|| {
            error!("Failed to set slot map key when adding event listener");
            panic!("Failed to set slot map key when adding event listener");
        });

        // Create and return the registration
        let lr = ListenerRegistration::new(Box::new(move || {
            info!("Deregistering listener for event {:?}", &event_key);

            inner_clone.listeners.alter(event_key, |listeners_opt| {
                match listeners_opt {
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
                }
            })
        }));

        listener_for_registration.add_listener_registration(lr);
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
