use crate::model::{Point, Rect};

define_event_bus!(
    EventBus,
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
