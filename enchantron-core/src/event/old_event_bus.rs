use std::any::{Any, TypeId};
use std::sync::Arc;

use super::{Event, ListenerRegistration};

use crate::util::BoxedAny;

use anymap::any::CloneAny;

use async_chashmap::CHashMap;

use futures::future::join_all;

use itertools::Itertools;

use rand::random;
use tokio::runtime::Handle;
use tokio::stream::StreamExt;
use tokio::sync::broadcast::{
    channel as broadcast_channel, Receiver as BroadcastReceiver,
    Sender as BroadcastSender,
};
use tokio::sync::mpsc::{
    unbounded_channel as mpsc_channel, UnboundedReceiver as MpscReceiver,
    UnboundedSender as MpscSender,
};

const MAIN_WORKER_COUNT: usize = 4;
const CHANNEL_BUFFER_SIZE: usize = 512;

/// Centeralizable component for blindly passing and receiving messages.
/// Internally this contains an arc, so this object can be cloned without worry
#[derive(Clone)]
pub struct EventBus {
    inner: Arc<InnerEventBus>,
}

/// Impl details for event bus
struct InnerEventBus {
    listeners: CHashMap<TypeId, BroadcastSender<BoxedAny>>,
    sinks: Vec<MpscSender<(TypeId, BoxedAny)>>,
    threadpool: Handle,
}

impl InnerEventBus {
    fn create(
        threadpool: Handle,
    ) -> (Vec<MpscReceiver<(TypeId, BoxedAny)>>, InnerEventBus) {
        let mut senders = Vec::new();
        let mut receivers = Vec::new();

        for _ in 0..MAIN_WORKER_COUNT {
            let (sender, receiver) = mpsc_channel();
            senders.push(sender);
            receivers.push(receiver);
        }

        (
            receivers,
            InnerEventBus {
                listeners: CHashMap::default(),
                sinks: senders,
                threadpool,
            },
        )
    }

    async fn post(&self, event_type: TypeId, event: BoxedAny) {
        self.listeners
            .with(&event_type, |sender_opt| {
                if let Some(sender) = sender_opt {
                    sender.send(event).expect("Faied to send");
                }
            })
            .await
    }
}

impl EventBus {
    pub fn new(tokio_runtime_handle: &Handle) -> EventBus {
        let (receivers, inner_event_bus_raw) =
            InnerEventBus::create(tokio_runtime_handle.clone());

        let inner_event_bus = Arc::new(inner_event_bus_raw);

        for mut receiver in receivers {
            let inner_clone = inner_event_bus.clone();

            inner_event_bus.threadpool.spawn(async move {
                while let Some((event_type, event)) = receiver.next().await {
                    inner_clone.post(event_type, event).await
                }
            });
        }

        EventBus {
            inner: inner_event_bus,
        }
    }

    /// Register the given event listener to listen for events of the given type, and
    /// return a registration that can be used to deregister the listener from this
    /// event type
    pub async fn register<E>(&self)
    // -> (ListenerRegistration, impl StreamExt<Item = E>)
    where
        E: Event,
    {
        let event_key = TypeId::of::<E>();

        let mut reciever_opt: Option<BroadcastReceiver<BoxedAny>> = None;

        {
            let receiver_ref = &mut reciever_opt;

            info!("Adding a listener for {:?}", &event_key);

            self.inner
                .listeners
                .alter(event_key, move |sender_opt| {
                    let (sender, receiver): (
                        BroadcastSender<BoxedAny>,
                        BroadcastReceiver<BoxedAny>,
                    ) = match sender_opt {
                        None => broadcast_channel(CHANNEL_BUFFER_SIZE),
                        Some(existing_sender) => {
                            let receiver = existing_sender.subscribe();
                            (existing_sender, receiver)
                        }
                    };

                    *receiver_ref = Some(receiver);
                    Some(sender)
                })
                .await;
        }

        let mut main_receiver =
            reciever_opt.expect("Failed to create receiver");
        let (sender, mut receiver) = mpsc_channel();

        let inner_clone = self.inner.clone();

        self.inner.threadpool.spawn(async move {
            while let Ok(event_any) = main_receiver.recv_ref(false) {
                let event = event_any
                    .map_value(|any_ref_opt| {
                        any_ref_opt.map(|any_ref| {
                            any_ref.downcast_ref::<E>().map(Clone::clone)
                        })
                    })
                    .expect("Wrong event type")
                    .clone();
                sender.send(event);
            }
        });

        unimplemented!();

        // // Create and return the registration
        // (
        //     ListenerRegistration::new(Box::new(move || {
        //         info!("Deregistering listener for event {:?}", &event_key);

        //         let another_clone = inner_clone.clone();

        //         inner_clone.threadpool.spawn(async move {

        //         another_clone
        //             .listeners
        //             .alter(event_key, |listeners_opt| match listeners_opt {
        //                 None => {
        //                     warn!(
        //                         "Attempted to remove a listener from an empty map"
        //                     );
        //                     None
        //                 }
        //                 Some(mut listeners) => {
        //                     let _ = listeners.remove(slot_map_key);
        //                     Some(listeners)
        //                 }
        //             })
        //             .await
        //     });
        //     })),
        //     receiver
        //         .map(|a| a.downcast_ref::<E>().expect("Invalid type").clone()),
        // )
    }

    pub fn post<E>(&self, event: E)
    where
        E: Event,
    {
        self.inner.sinks[random::<usize>() % MAIN_WORKER_COUNT]
            .send((TypeId::of::<E>(), Box::new(event)));
    }
}
