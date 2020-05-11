use crate::model::{Point, Rect};
use crate::ui::{DragEventType, DragPoint, ViewportInfo};

define_event_bus!(
    EventBus,
    LoadResources{},
    StartGame{ pub new: bool },
    Layout{
        pub width: i64,
        pub height: i64,
    },
    ViewportChange{ pub new_viewport: ViewportInfo },
    DragEvent{
        pub state: DragEventType,
        pub drag_point_1: DragPoint,
        pub drag_point_2_opt: Option<DragPoint>
    },
    Magnify{
        pub scale_change_additive: f64,
        pub global_center: Point
    },
    EnteredViewport{

    },
    ExitedViewport{

    }
);
