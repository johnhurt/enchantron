macro_rules! define_event_bus {
    ($event_bus_name:ident, $($e:ident $body:tt ), *) => {

        pub type $event_bus_name = hidden::EventBus;

        mod hidden {
            #![allow(non_snake_case)]

            use crate::event::Event;
            use std::sync::Arc;
            use std::future::Future;
            use futures::future::select;

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
            use tokio::sync::oneshot::{
                channel as oneshot_channel,
                Sender as OneshotSender,
                Receiver as OneshotReceiver
            };

            #[derive(Clone)]
            pub struct EventBus {
                inner: Arc<Inner>
            }

            struct Inner {
                runtime_handle: Handle,
                senders: Senders
            }

            struct Senders {
                $(
                    $e : BroadcastSender<super::$e>,
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

                        let _ = self.inner.senders.$e.send(event);
                    }
                )*


                pub fn register<E: EventBusEventTrait>(&self)
                    -> impl StreamExt<Item = E> where E: Event
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
                        senders: Senders {$(
                            $e,
                        )*}

                    }
                }

                $(
                    fn $e(
                        self: Arc<Inner>,
                        mut main_receiver: BroadcastReceiver<super::$e>
                    )
                        -> MpscReceiver<super::$e>
                    {
                        info!("Registering a listener for {} events",
                            stringify!($e));

                        let (mut sender, receiver) = mpsc_channel(512);

                        let _ = self.runtime_handle.spawn(async move {
                            while let Ok(event) = main_receiver.recv().await {
                                let _ = sender.send(event).await;
                            }

                            debug!("Receiver for {} closed", stringify!($e));
                        });

                        receiver
                    }
                )*

            }

            trait Registerable<E: Event> {


            }

            pub trait EventBusEventTrait : Event {
                fn post(self, event_bus: &EventBus);
                fn register(event_bus: &EventBus) -> MpscReceiver<Self>;
            }

            $(
                impl Event for super::$e {}

                impl EventBusEventTrait for super::$e {
                    fn post(self, event_bus: &EventBus) {
                        event_bus.$e(self)
                    }

                    fn register(event_bus: &EventBus) -> MpscReceiver<super::$e> {
                        Inner::$e(
                            event_bus.inner.clone(),
                            event_bus.inner.senders.$e.subscribe())
                    }
                }
            )*
        }

        $(

        #[derive(Debug, Clone, Default)]
        pub struct $e $body

        )*

    }
}
