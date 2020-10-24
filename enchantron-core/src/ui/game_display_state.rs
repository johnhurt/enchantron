use crate::model::{Point, Rect, Size};
use crate::ui::{DragTracker, ViewportInfo};

pub struct GameDisplayState {
    pub viewport_info: ViewportInfo,
    pub drag_tracker: DragTracker,
}

#[allow(dead_code)]
impl GameDisplayState {
    pub fn new() -> GameDisplayState {
        GameDisplayState {
            viewport_info: Default::default(),
            drag_tracker: Default::default(),
        }
    }

    // Get a reference to the viewport rectangle
    pub fn get_viewport_rect(&self) -> &'_ Rect {
        &self.viewport_info.viewport_rect
    }

    // Get a reference to the top-left corner of the viewport rectangle
    pub fn get_viewport_top_left(&self) -> &'_ Point {
        &self.get_viewport_rect().top_left
    }

    pub fn get_viewport_scale(&self) -> f64 {
        self.viewport_info.viewport_scale
    }

    /// Update the layout of the display based on a change in the size of
    /// screen
    pub fn layout(&mut self, new_size: Size) -> &ViewportInfo {
        self.viewport_info.resize_screen(new_size);
        &self.viewport_info
    }

    /// change the scale of the area shown by the viewport by the given
    /// additive amount, and return the new scale. The center of the zoom
    pub fn change_scale_additive_around_center_point(
        &mut self,
        scale_change_additive: f64,
        magnify_center_screen_point: Point,
    ) -> &ViewportInfo {
        self.viewport_info
            .change_scale_additive_around_center_point(
                scale_change_additive,
                magnify_center_screen_point,
            );

        &self.viewport_info
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

    /// Move the viewport rect's top left to the given point and return a
    /// ref to the resulting top_left
    pub fn move_viewport(&mut self, new_top_left: Point) -> &'_ ViewportInfo {
        self.viewport_info.move_viewport(new_top_left);

        &self.viewport_info
    }

    pub fn move_viewport_by(
        &mut self,
        delta_top_left: Point,
    ) -> &'_ ViewportInfo {
        let new_top_left =
            &self.viewport_info.viewport_rect.top_left + delta_top_left;

        self.viewport_info.move_viewport(new_top_left);

        &self.viewport_info
    }
}
