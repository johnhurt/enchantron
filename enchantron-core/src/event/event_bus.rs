use std::any::Any;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Weak};

use super::{Event, EventKey, EventListener, ListenerRegistration};

use crate::util::SimpleSlotMap;

use atomic_counter::{AtomicCounter, RelaxedCounter};
use crossbeam_channel;
use crossbeam_channel::{Receiver, Sender};

use chashmap::CHashMap;
use rayon;
use rayon::{ThreadPool, ThreadPoolBuilder};

const WORKER_COUNT: usize = 8;

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
    registration_thread_pool: ThreadPool,
}

impl<K: EventKey> Default for EventBus<K> {
    fn default() -> EventBus<K> {
        let (inner_event_bus, sources): (
            InnerEventBus<K>,
            Vec<Receiver<(K, BoxedAny)>>,
        ) = InnerEventBus::create();

        let inner_event_bus_arc = Arc::new(inner_event_bus);

        let pool = ThreadPoolBuilder::new()
            .num_threads(WORKER_COUNT)
            .build()
            .unwrap();

        sources.into_iter().enumerate().for_each(|(i, source)| {
            let copied_event_bus = inner_event_bus_arc.clone();

            pool.spawn(move || loop {
                debug!("Event loop {} looping", i);

                match source.recv() {
                    Ok((key, arg)) => {
                        debug!("Firing {:?} - {:?}", key, arg);

                        copied_event_bus.evaluate(key, arg);
                        debug!("Fired {:?}", key);
                    }
                    Err(_) => info!("Eventbus channel {} closed", i),
                };
            })
        });

        EventBus {
            inner: inner_event_bus_arc,
        }
    }
}

impl<K: EventKey> InnerEventBus<K> {
    fn create() -> (InnerEventBus<K>, Vec<Receiver<(K, BoxedAny)>>) {
        let mut sinks: Vec<Sender<(K, BoxedAny)>> = Vec::new();
        let mut sources: Vec<Receiver<(K, BoxedAny)>> = Vec::new();

        for _ in 0..WORKER_COUNT {
            let (sink, source): (
                Sender<(K, BoxedAny)>,
                Receiver<(K, BoxedAny)>,
            ) = crossbeam_channel::unbounded();
            sinks.push(sink);
            sources.push(source);
        }

        (
            InnerEventBus {
                listeners: CHashMap::default(),
                sinks: sinks,
                event_counter: RelaxedCounter::new(0),
                registration_thread_pool: ThreadPoolBuilder::new()
                    .num_threads(WORKER_COUNT)
                    .build()
                    .unwrap(),
            },
            sources,
        )
    }

    fn evaluate(&self, key: K, arg: BoxedAny) {
        debug!("Evaluating");

        if let Some(handlers) = self.listeners.get(&key) {
            handlers.iter().for_each(|func| func(&*arg)); // <- Note the deref before borrow
        } else {
            info!("No handlers found for event key: {:?}", key);
        }
        debug!("Evaluated");
    }
}

impl<K: EventKey> EventBus<K> {
    /// Create a new and empty default event bus
    pub fn new() -> EventBus<K> {
        EventBus::default()
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
            }
            else {
                return
            }
        };

        let inner_clone = self.inner.clone();

        self.inner.registration_thread_pool.spawn(move || {
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

                let another_inner_clone = inner_clone.clone();

                inner_clone.registration_thread_pool.spawn(move || {

                    another_inner_clone.listeners.alter(event_key, |listeners_opt| {
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
                });
            }));

            listener_for_registration.add_listener_registration(lr);
        });

        debug!("Finished registering blah");
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
