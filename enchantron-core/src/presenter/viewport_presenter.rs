use crate::event::*;
use crate::model::{Point, Rect, Size};
use crate::ui::{
    HasMutableLocation, HasMutableScale, PanZoomEvent, PanZoomTracker,
    TouchEvent, ViewportInfo,
};
use crate::view_types::ViewTypes;

pub struct ViewportPresenter<T: ViewTypes> {
    pub viewport: T::Viewport,
    pub event_bus: EventBus,
    pub viewport_info: ViewportInfo,
    pub touch_tracker: PanZoomTracker,
}

impl<T> ViewportPresenter<T>
where
    T: ViewTypes,
{
    pub fn new(
        viewport: T::Viewport,
        event_bus: EventBus,
    ) -> ViewportPresenter<T> {
        ViewportPresenter {
            viewport,
            event_bus,
            viewport_info: ViewportInfo::default(),
            touch_tracker: PanZoomTracker::default(),
        }
    }

    pub fn on_layout(&mut self, layout_event: &Layout) {
        self.layout(layout_event.size, layout_event.scale);

        self.event_bus.post(ViewportChange::new(self.viewport_info));

        self.viewport
            .set_location_point(&self.viewport_info.viewport_rect.top_left);
    }

    pub fn on_magnify(&mut self, magnify_event: &Magnify) {
        let Magnify {
            scale_change_additive,
            global_center:
                Point {
                    x: zoom_center_x,
                    y: zoom_center_y,
                },
        } = magnify_event;

        trace!("Scale changing by {}", scale_change_additive);

        let magnify_center_screen_point =
            Point::new(*zoom_center_x, *zoom_center_y);

        self.change_scale_additive_around_center_point(
            *scale_change_additive,
            magnify_center_screen_point,
        );

        self.event_bus.post(ViewportChange::new(self.viewport_info));

        self.viewport.set_scale_and_location_point(
            self.viewport_info.viewport_scale,
            &self.viewport_info.viewport_rect.top_left,
        );
    }

    pub fn on_touch_event(&mut self, touch_event: &TouchEvent) {
        let pan_zoom_event = self.touch_tracker.to_pan_zoom_event(*touch_event);

        match pan_zoom_event {
            Some(PanZoomEvent::Move(drag_move)) => self.on_drag_move(drag_move),
            Some(PanZoomEvent::MoveAndScale(drag_move, scale)) => {
                self.on_drag_move_and_scale(drag_move, scale)
            }
            _ => (),
        }
    }

    pub fn get_viewport_scale(&self) -> f64 {
        self.viewport_info.viewport_scale
    }

    /// Update the layout of the display based on a change in the size of
    /// screen
    pub fn layout(&mut self, new_size: Size, scale: f64) {
        self.viewport_info.resize_screen(new_size, scale);
    }

    /// change the scale of the area shown by the viewport by the given
    /// additive amount, and return the new scale. The center of the zoom
    pub fn change_scale_additive_around_center_point(
        &mut self,
        scale_change_additive: f64,
        magnify_center_screen_point: Point,
    ) {
        self.viewport_info
            .change_scale_additive_around_center_point(
                scale_change_additive,
                magnify_center_screen_point,
            );
    }

    /// change the scale of the area shown by the viewport by the given
    /// additive amount, move by the given amount, and return the new scale.
    pub fn change_scale_and_move(
        &mut self,
        scale: f64,
        position_shift: Point,
    ) -> &'_ ViewportInfo {
        self.viewport_info
            .change_scale_and_move(scale, position_shift);

        &self.viewport_info
    }

    pub fn move_viewport_by(&mut self, delta_top_left: Point) {
        let new_top_left =
            self.viewport_info.viewport_rect.top_left + delta_top_left;

        self.viewport_info.move_viewport(new_top_left);
    }

    fn on_drag_move(&mut self, drag_move: Point) {
        let scale = self.get_viewport_scale();

        let position_shift = drag_move * scale;

        self.move_viewport_by(position_shift);

        self.event_bus.post(ViewportChange::new(self.viewport_info));

        let new_position_ref = &self.viewport_info.viewport_rect.top_left;

        self.viewport.set_location_point(new_position_ref);
    }

    fn on_drag_move_and_scale(&mut self, drag_move: Point, new_scale: f64) {
        self.change_scale_and_move(new_scale, drag_move);

        self.event_bus.post(ViewportChange::new(self.viewport_info));

        self.viewport.set_scale_and_location_point(
            self.viewport_info.viewport_scale,
            &self.viewport_info.viewport_rect.top_left,
        );
    }
}
