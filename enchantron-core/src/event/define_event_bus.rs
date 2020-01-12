macro_rules! define_event_bus {
    ($event_bus_name:ident, $($e:ident $body:tt ), *) => {

        pub use hidden::EventBus as $event_bus_name;

        mod hidden {

            use crate::event::Event;
            use std::sync::Arc;

            pub struct EventBus {
                inner: Arc<Inner>
            }

            struct Inner {

            }

            impl EventBus {

                pub fn post(&self, ) {}

            }


            #[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
            enum EventKey {
            $(
                $e,
            )*
            }

            trait GetEventKeys {
                fn get_event_key(&self) -> EventKey;
            }

            $(
                impl Event for $e {}
                impl GetEventKeys for $e {
                    fn get_event_key(&self) -> EventKey { EventKey::$e }
                }
            )*
        }

        $(

        #[derive(Debug, Clone, Default)]
        pub struct $e $body

        )*

    }
}
