macro_rules! define_event_bus {
    ($event_bus_name:ident, $($e:ident $body:tt ), *) => {

        pub type $event_bus_name = event_bus_hidden::EventBus;
        pub use event_bus_hidden::Event;

        mod event_bus_hidden {
            #![allow(non_snake_case)]

            use crate::event::{ ListenerRegistration };
            use std::sync::Arc;
            use std::future::Future;
            use std::fmt::Debug;

            use futures::future::FutureExt;

            use atomic_counter::{ AtomicCounter, ConsistentCounter };

            use tokio::stream::StreamExt;
            use tokio::runtime::Handle;
            use tokio::task::JoinHandle;
            use tokio::sync::broadcast::{
                channel as broadcast_channel,
                Sender as BroadcastSender,
                Receiver as BroadcastReceiver
            };
            use tokio::sync::oneshot::{
                channel as oneshot_channel,
                Sender as OneshotSender,
                Receiver as OneshotReceiver
            };
            use tokio::sync::watch::{
                channel as watch_channel,
                Sender as WatchSender,
                Receiver as WatchReceiver
            };
            use tokio::sync::mpsc::{
                channel as mpsc_channel,
                Receiver as MpscReceiver
            };

            #[derive(Clone)]
            pub struct EventBus {
                inner: Arc<Inner>
            }

            pub struct Inner {
                runtime_handle: Handle,
                registration_counter: ConsistentCounter,
                senders: Senders,
            }

            struct Senders {
                $(
                    $e : BroadcastSender<$e>,
                )*
            }


            impl EventBus {

                pub fn new(runtime_handle: &Handle) -> EventBus {
                    EventBus {
                        inner: Arc::new(Inner::new(runtime_handle.clone()))
                    }
                }

                pub fn post<E: Event>(&self, event: E) {
                    event.post(self)
                }

                $(
                    fn $e(&self, event: super::$e) {
                        debug!("Posting {} event: {:?}", stringify!($e), event);

                        let _ = self.inner.senders.$e.send(event);
                    }
                )*


                pub fn register<E>(&self)
                    -> (ListenerRegistration, impl StreamExt<Item = E>)
                where E: Event
                {
                    E::register(self)
                }

                pub fn register_for_one<E>(&self)
                    -> impl Future<Output = Option<E>>
                where E: Event
                {
                    E::register_for_one(self).map(Result::ok)
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

            mod register {
                pub use super::*;


                $(
                pub fn $e(
                    this: Arc<Inner>,
                    mut main_receiver: BroadcastReceiver<super::super::$e>
                )
                    -> ( ListenerRegistration, MpscReceiver<super::super::$e> )
                {
                    info!("Registering a listener for {} events",
                        stringify!($e));

                    let listener_id = this.registration_counter.inc();

                    let (mut end_sender, end_receiver) = oneshot_channel::<()>();
                    let (mut )

                    let _ = this.runtime_handle.spawn(async move {

                        loop {
                            futures::select! {
                                event = main_receiver => {
                                    let _ = sender.send(event).await
                                },
                                _ = end_receiver => break
                            }
                        }

                        debug!("Receiver for {} closed", stringify!($e));
                    });

                    let deregister_inner = this.clone();


                    let listener_registration = ListenerRegistration::new(
                        Box::new(move || {
                            end_sender.send(());
                        })
                    );

                    (listener_registration, receiver)
                }
            )*}

            mod register_for_one {
                pub use super::*;
                $(
                    pub fn $e(this: Arc<Inner>,
                        mut main_receiver: BroadcastReceiver<$e>) -> OneshotReceiver<super::super::$e>{
                            todo!();
                        }
                )*
            }

            impl Inner {

                fn new(runtime_handle: Handle) -> Inner {
                    let senders = Senders {
                        $(
                            $e: broadcast_channel(1024).0,
                        )*
                    };


                    Inner{
                        runtime_handle,
                        registration_counter: Default::default(),
                        senders,
                    }
                }

            }

            type ListenerKey = usize;

            pub trait Event: Unpin + Send + Debug + Clone + 'static {
                fn post(self, event_bus: &EventBus);
                fn register(event_bus: &EventBus)
                    -> ( ListenerRegistration, MpscReceiver<Self> );
                fn register_for_one(event_bus: &EventBus)
                    -> OneshotReceiver<Self>;
            }

            $(
                impl Event for super::$e {
                    fn post(self, event_bus: &EventBus) {
                        event_bus.$e(self)
                    }

                    fn register(event_bus: &EventBus)
                        -> ( ListenerRegistration, MpscReceiver<super::$e> )
                    {
                        register::$e(
                            event_bus.inner.clone(),
                            event_bus.inner.senders.$e.subscribe())
                    }

                    fn register_for_one(event_bus: &EventBus)
                        -> OneshotReceiver<super::$e>
                    {
                        register_for_one::$e(
                            event_bus.inner.clone(),
                            event_bus.inner.senders.$e.subscribe())
                    }
                }
            )*
        }

        $(

        #[derive(Debug, Clone, derive_new::new)]
        pub struct $e $body

        )*

    }
}
