use crate::model::{Point, Size};
use crate::ui::{RawTouch, TouchEventType, ViewportInfo};

define_event_bus!(
    EventBus,
    LoadResources{},
    TerrainPresenterStarted{},
    StartGame{ pub new: bool },
    StopGameRequested{},
    GameStopped{},
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
            size: Size,
            scale: f64
        },
        RawTouchEvent {
            state: TouchEventType,
            touch: RawTouch,
            other_touch_opt: Option<RawTouch>
        },
        Magnify {
            scale_change_additive: f64,
            global_center: Point
        }
    }
);

impl RawTouchEvent {
    pub fn start_1(touch: RawTouch) -> Self {
        RawTouchEvent::new(TouchEventType::Start, touch, None)
    }

    pub fn start_2(touch_1: RawTouch, touch_2: RawTouch) -> Self {
        RawTouchEvent::new(TouchEventType::Start, touch_1, Some(touch_2))
    }

    pub fn move_1(touch: RawTouch) -> Self {
        RawTouchEvent::new(TouchEventType::Move, touch, None)
    }

    pub fn move_2(touch_1: RawTouch, touch_2: RawTouch) -> Self {
        RawTouchEvent::new(TouchEventType::Move, touch_1, Some(touch_2))
    }

    pub fn end_1(touch: RawTouch) -> Self {
        RawTouchEvent::new(TouchEventType::End, touch, None)
    }

    pub fn end_2(touch_1: RawTouch, touch_2: RawTouch) -> Self {
        RawTouchEvent::new(TouchEventType::End, touch_1, Some(touch_2))
    }
}
