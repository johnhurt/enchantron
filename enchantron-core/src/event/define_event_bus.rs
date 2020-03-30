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

            use tokio::stream::{
                StreamExt,
                StreamMap
            };
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
                senders: Senders,
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
                    let main_receiver = E::get_main_receiver(self);
                    let (end_sender, end_receiver)
                        = mpsc_channel::<Option<E>>(1);

                    let inner_clone = self.inner.clone();

                    let listener_registration = ListenerRegistration::new(
                        Box::new(move || {
                            let mut end_sender = end_sender;
                            inner_clone.runtime_handle.spawn(async move {
                                let _ = end_sender.send(None).await;
                            });
                        })
                    );

                    let result_stream = main_receiver
                        .map(Result::ok)
                        .merge(end_receiver)
                        .take_while(Option::is_some)
                        .map(Option::unwrap);

                    (listener_registration, result_stream)
                }

                pub fn register_for_one<E>(&self)
                    -> impl Future<Output = Option<E>>
                where E: Event
                {
                    let main_receiver = E::get_main_receiver(self);

                    async move {
                        main_receiver
                            .take_while(Result::is_ok)
                            .map(Result::unwrap)
                            .next()
                            .await
                    }
                }

                pub fn register_to_watch<E>(&self)
                    -> (ListenerRegistration, impl StreamExt<Item = E>)
                where E: Event {
                    let (listener_registration, mut main_stream)
                        = self.register::<E>();

                    let (watch_sender, watch_receiver)
                        = watch_channel::<Option<E>>(None);

                    self.spawn(async move {
                        while let Some(e) = main_stream.next().await {
                            let _ = watch_sender.broadcast(Some(e));
                        }
                    });

                    let result_stream = watch_receiver
                        .skip(1)
                        .take_while(Option::is_some)
                        .map(Option::unwrap);

                    (listener_registration, result_stream)
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
                    let senders = Senders {
                        $(
                            $e: broadcast_channel::<super::$e>(1024).0,
                        )*
                    };


                    Inner{
                        runtime_handle,
                        senders,
                    }
                }

            }

            type ListenerKey = usize;

            pub trait Event: Unpin + Send + Sync + Debug + Clone + 'static {
                fn post(self, event_bus: &EventBus);
                fn get_main_receiver(event_bus: &EventBus) -> BroadcastReceiver<Self>;
            }

            $(
                impl Event for super::$e {
                    fn post(self, event_bus: &EventBus) {
                        event_bus.$e(self)
                    }

                    fn get_main_receiver(event_bus: &EventBus)
                        -> BroadcastReceiver<super::$e>
                    {
                        event_bus.inner.senders.$e.subscribe()
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
