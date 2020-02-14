use crate::model::{Point, Rect};
use crate::ui::ViewportInfo;

#[derive(Clone, Debug)]
pub enum DragEventType {
    Start,
    Move,
    End,
}

define_event_bus!(
    EventBus,
    LoadResources{},
    StartGame{ pub new: bool },
    Layout{
        pub width: i64,
        pub height: i64,
    },
    ViewportChange{ pub new_viewport: ViewportInfo },
    Drag{
        pub state: DragEventType,
        pub global_point: Point,
        pub local_point: Point,
    },
    Magnify{
        pub scale_change_additive: f64,
        pub global_center: Point
    }
);
