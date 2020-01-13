macro_rules! define_event_bus {
    ($event_bus_name:ident, $($e:ident $body:tt ), *) => {

        pub type $event_bus_name = hidden::EventBus;

        mod hidden {
            #![allow(non_snake_case)]

            use crate::event::Event;
            use std::sync::Arc;

            use tokio::runtime::Handle;
            use tokio::sync::broadcast::{
                channel as broadcast_channel,
                Sender as BroadcastSender,
                Receiver as BroadcastReceiver
            };
            use tokio::sync::mpsc::{
                channel as mpsc_channel,
                Sender as MpscSender,
                Receiver as MpscReceiver
            };

            #[derive(Clone)]
            pub struct EventBus {
                inner: Arc<Inner>
            }

            struct Inner {
                runtime_handle: Handle,

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

                pub fn post<E: Postable>(&self, event: E) {
                    event.post(self)
                }

                $(
                    fn $e(&self, event: super::$e) {
                        let _ = self.inner.$e.send(event);
                    }
                )*


                pub fn register<E>() -> MpscReceiver<E> where E: Event {

                }
            }

            impl Inner {

                fn new(runtime_handle: Handle) -> Inner {
                    $(
                        let ($e, _) = broadcast_channel(128);
                    )*

                    Inner{
                        runtime_handle,
                        $(
                            $e,
                        )*

                    }
                }


            }

            pub trait Postable : Event {
                fn post(self, event_bus: &EventBus);
            }

            $(
                impl Event for super::$e {}
                impl Postable for super::$e {
                    fn post(self, event_bus: &EventBus) {
                        event_bus.$e(self)
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
