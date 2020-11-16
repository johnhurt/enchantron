use super::{RawTouch, RawTouchPoint, TouchEventType};
use crate::event::RawTouchEvent;

pub type TouchFn = dyn Fn(RawTouchEvent) + 'static + Send;

pub struct MultiTouchHandler {
    multi_drag_fn: Box<TouchFn>,
}

impl MultiTouchHandler {
    pub fn new(
        drag_handler: impl Fn(RawTouchEvent) + 'static + Send,
    ) -> MultiTouchHandler {
        MultiTouchHandler {
            multi_drag_fn: Box::new(drag_handler),
        }
    }

    pub fn on_one_drag_start(
        &self,
        drag_id: i64,
        global_x: f64,
        global_y: f64,
        local_x: f64,
        local_y: f64,
        click_count: i64,
    ) {
        (self.multi_drag_fn)(RawTouchEvent::new(
            TouchEventType::Start,
            RawTouch {
                touch_id: drag_id,
                point: RawTouchPoint::new(global_x, global_y, local_x, local_y),
                click_count: click_count as u8,
            },
            None,
        ));
    }

    pub fn on_one_drag_move(
        &self,
        drag_id: i64,
        global_x: f64,
        global_y: f64,
        local_x: f64,
        local_y: f64,
        click_count: i64,
    ) {
        (self.multi_drag_fn)(RawTouchEvent::new(
            TouchEventType::Move,
            RawTouch {
                touch_id: drag_id,
                point: RawTouchPoint::new(global_x, global_y, local_x, local_y),
                click_count: click_count as u8,
            },
            None,
        ));
    }

    pub fn on_one_drag_end(
        &self,
        drag_id: i64,
        global_x: f64,
        global_y: f64,
        local_x: f64,
        local_y: f64,
        click_count: i64,
    ) {
        (self.multi_drag_fn)(RawTouchEvent::new(
            TouchEventType::End,
            RawTouch {
                touch_id: drag_id,
                point: RawTouchPoint::new(global_x, global_y, local_x, local_y),
                click_count: click_count as u8,
            },
            None,
        ));
    }

    #[allow(clippy::too_many_arguments)]
    pub fn on_two_drags_start(
        &self,
        drag_id_1: i64,
        global_x_1: f64,
        global_y_1: f64,
        local_x_1: f64,
        local_y_1: f64,
        click_count_1: i64,
        drag_id_2: i64,
        global_x_2: f64,
        global_y_2: f64,
        local_x_2: f64,
        local_y_2: f64,
        click_count_2: i64,
    ) {
        (self.multi_drag_fn)(RawTouchEvent::new(
            TouchEventType::Start,
            RawTouch {
                touch_id: drag_id_1,
                point: RawTouchPoint::new(
                    global_x_1, global_y_1, local_x_1, local_y_1,
                ),
                click_count: click_count_1 as u8,
            },
            Some(RawTouch {
                touch_id: drag_id_2,
                point: RawTouchPoint::new(
                    global_x_2, global_y_2, local_x_2, local_y_2,
                ),
                click_count: click_count_2 as u8,
            }),
        ));
    }

    #[allow(clippy::too_many_arguments)]
    pub fn on_two_drags_move(
        &self,
        drag_id_1: i64,
        global_x_1: f64,
        global_y_1: f64,
        local_x_1: f64,
        local_y_1: f64,
        click_count_1: i64,
        drag_id_2: i64,
        global_x_2: f64,
        global_y_2: f64,
        local_x_2: f64,
        local_y_2: f64,
        click_count_2: i64,
    ) {
        (self.multi_drag_fn)(RawTouchEvent::new(
            TouchEventType::Move,
            RawTouch {
                touch_id: drag_id_1,
                point: RawTouchPoint::new(
                    global_x_1, global_y_1, local_x_1, local_y_1,
                ),
                click_count: click_count_1 as u8,
            },
            Some(RawTouch {
                touch_id: drag_id_2,
                point: RawTouchPoint::new(
                    global_x_2, global_y_2, local_x_2, local_y_2,
                ),
                click_count: click_count_2 as u8,
            }),
        ));
    }

    #[allow(clippy::too_many_arguments)]
    pub fn on_two_drags_end(
        &self,
        drag_id_1: i64,
        global_x_1: f64,
        global_y_1: f64,
        local_x_1: f64,
        local_y_1: f64,
        click_count_1: i64,
        drag_id_2: i64,
        global_x_2: f64,
        global_y_2: f64,
        local_x_2: f64,
        local_y_2: f64,
        click_count_2: i64,
    ) {
        (self.multi_drag_fn)(RawTouchEvent::new(
            TouchEventType::End,
            RawTouch {
                touch_id: drag_id_1,
                point: RawTouchPoint::new(
                    global_x_1, global_y_1, local_x_1, local_y_1,
                ),
                click_count: click_count_1 as u8,
            },
            Some(RawTouch {
                touch_id: drag_id_2,
                point: RawTouchPoint::new(
                    global_x_2, global_y_2, local_x_2, local_y_2,
                ),
                click_count: click_count_2 as u8,
            }),
        ));
    }
}
