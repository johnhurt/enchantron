use super::Event;

macro_rules! define_events {
    ($events_name:ident, $($e:ident $body:tt ), *) => {

        #[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
        pub enum $events_name {
        $(
            $e,
        )*
        }


        $(

        #[derive(Debug, Clone, Default)]
        pub struct $e $body

        impl Event<$events_name> for  $e {
            fn get_event_key(&self) -> $events_name { $events_name::$e }
        }

        )*
    }
}

define_events!(EnchantronEvent,
    LoadResources{},
    StartGame{ pub new: bool },
    Layout{
      pub width: i64,
      pub height: i64,
    }
);
