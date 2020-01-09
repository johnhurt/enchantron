use super::Event;
use crate::model::{Point, Rect};

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

        impl Event<$events_name> for $e {
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
    },
    ViewportChange{ pub new_viewport_rect: Rect },
    DragStart{
        pub global_point: Point,
        pub local_point: Point,
    },
    DragMove{
        pub global_point: Point,
        pub local_point: Point,
    },
    DragEnd{
        pub global_point: Point,
        pub local_point: Point,
    },
    Magnify{
        pub scale_change_additive: f64,
        pub global_center: Point
    }
);
