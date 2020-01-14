macro_rules! define_event_bus {
    ($event_bus_name:ident, $($e:ident $body:tt ), *) => {

        pub type $event_bus_name = event_bus_hidden::EventBus;

        mod event_bus_hidden {
            #![allow(non_snake_case)]

            use crate::event::{ Event, ListenerRegistration };
            use std::sync::Arc;
            use std::future::Future;

            use atomic_counter::{ AtomicCounter, ConsistentCounter };

            use tokio::stream::StreamExt;
            use tokio::runtime::Handle;
            use tokio::task::JoinHandle;
            use tokio::sync::broadcast::{
                channel as broadcast_channel,
                Sender as BroadcastSender,
                Receiver as BroadcastReceiver
            };
            use tokio::sync::mpsc::{
                channel as mpsc_channel,
                Receiver as MpscReceiver
            };

            #[derive(Clone)]
            pub struct EventBus {
                inner: Arc<Inner>
            }

            struct Inner {
                runtime_handle: Handle,
                registration_counter: ConsistentCounter,
                senders: Senders
            }

            struct Senders {
                $(
                    $e : BroadcastSender<event_bus_enums::$e>,
                )*
            }

            impl EventBus {

                pub fn new(runtime_handle: &Handle) -> EventBus {
                    EventBus {
                        inner: Arc::new(Inner::new(runtime_handle.clone()))
                    }
                }

                pub fn post<E: EventBusEventTrait>(&self, event: E) {
                    event.post(self)
                }

                $(
                    fn $e(&self, event: super::$e) {
                        debug!("Posting {} event: {:?}", stringify!($e), event);

                        let _ = self.inner.senders.$e.send(
                            event_bus_enums::$e::Event(event));
                    }
                )*


                pub fn register<E: EventBusEventTrait>(&self)
                    -> (ListenerRegistration, impl StreamExt<Item = E>)
                where E: Event
                {
                    E::register(self)
                }

                /// Convienient passthrough to the tokio spawner
                pub fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
                where
                    F: Future + Send + 'static,
                    F::Output: Send + 'static,
                {
                    self.inner.runtime_handle.spawn(future)
                }
            }

            impl Inner {

                fn new(runtime_handle: Handle) -> Inner {
                    $(
                        let ($e, _) = broadcast_channel(128);
                    )*

                    Inner{
                        runtime_handle,
                        registration_counter: Default::default(),
                        senders: Senders {$(
                            $e,
                        )*}

                    }
                }

                $(
                    fn $e(
                        self: Arc<Inner>,
                        mut main_receiver: BroadcastReceiver<event_bus_enums::$e>
                    )
                        -> ( ListenerRegistration, MpscReceiver<super::$e> )
                    {
                        info!("Registering a listener for {} events",
                            stringify!($e));

                        let listener_id = self.registration_counter.inc();

                        let (mut sender, receiver) = mpsc_channel(512);

                        let _ = self.runtime_handle.spawn(async move {
                            while let Ok(event_enum) = main_receiver.recv().await {
                                match event_enum {
                                    event_bus_enums::$e::Event(event) => {
                                        let _ = sender.send(event).await;
                                    },
                                    event_bus_enums::$e::ListenerClosed(closed_listener_id) => {
                                        if listener_id == closed_listener_id {
                                            break;
                                        }
                                    }
                                }
                            }

                            debug!("Receiver for {} closed", stringify!($e));
                        });

                        let deregister_inner = self.clone();


                        let listener_registration = ListenerRegistration::new(
                            Box::new(move || {
                                let deregister_sender = deregister_inner.senders.$e.clone();
                                let _ = deregister_inner.runtime_handle.spawn(async move {
                                    let _ = deregister_sender.send(
                                        event_bus_enums::$e::ListenerClosed(
                                            listener_id));
                                });
                            })
                        );

                        (listener_registration, receiver)
                    }
                )*

            }

            type ListenerKey = usize;

            mod event_bus_enums {
                $(
                    #[derive(Clone, Debug)]
                    pub enum $e {
                        Event(super::super::$e),
                        ListenerClosed(super::ListenerKey)
                    }
                )*
            }

            pub trait EventBusEventTrait : Event {
                fn post(self, event_bus: &EventBus);
                fn register(event_bus: &EventBus)
                    -> ( ListenerRegistration, MpscReceiver<Self> );
            }

            $(
                impl Event for super::$e {}

                impl EventBusEventTrait for super::$e {
                    fn post(self, event_bus: &EventBus) {
                        event_bus.$e(self)
                    }

                    fn register(event_bus: &EventBus)
                        -> ( ListenerRegistration, MpscReceiver<super::$e> )
                    {
                        Inner::$e(
                            event_bus.inner.clone(),
                            event_bus.inner.senders.$e.subscribe())
                    }
                }
            )*
        }

        $(

        #[derive(Debug, Clone)]
        pub struct $e $body

        )*

    }
}
