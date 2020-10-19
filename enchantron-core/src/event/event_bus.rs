use crate::model::Point;
use crate::ui::{DragEventType, DragPoint, ViewportInfo};

define_event_bus!(
    EventBus,
    LoadResources{},
    StartGame{ pub new: bool },
    ExitGame{},
    UI{ pub event: UIEvent },
    ViewportChange{ pub new_viewport: ViewportInfo }
);

macro_rules! define_ui_event {
    ($ui_event:ident { $( $event_type:ident { $( $field:ident : $field_type:ty ),* }  ),+ } ) => {

        #[derive(Copy, Clone, Debug)]
        pub enum $ui_event { $(
            $event_type {
                event: $event_type
            }
        ),+ }

        $(
            #[derive(Clone, Debug, Copy, derive_new::new)]
            pub struct $event_type { $(
                pub $field: $field_type
            ),* }

            impl From<$event_type> for UI {
                fn from(val: $event_type) -> UI {
                    UI {
                        event: UIEvent::$event_type {
                            event: val
                        }
                    }
                }
            }
        )+
    }
}

define_ui_event!(
    UIEvent {
        Layout {
            width: i64,
            height: i64
        },
        DragEvent {
            state: DragEventType,
            drag_point_1: DragPoint,
            drag_point_2_opt: Option<DragPoint>
        },
        Magnify {
            scale_change_additive: f64,
            global_center: Point
        }
    }
);

// #[derive(Clone, Debug)]
// pub enum UiEvent {
//     Layout {
//         width: i64,
//         height: i64,
//     },
//     DragEvent {
//         state: DragEventType,
//         drag_point_1: DragPoint,
//         drag_point_2_opt: Option<DragPoint>,
//     },
//     Magnify {
//         scale_change_additive: f64,
//         global_center: Point,
//     },
// }
