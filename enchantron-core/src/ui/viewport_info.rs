use crate::model::{Point, Rect, Size};

#[derive(Clone, Debug)]
pub struct ViewportInfo {
    pub viewport_rect: Rect,
    pub screen_size: Size,
    pub viewport_scale: f64,
}

impl Default for ViewportInfo {
    fn default() -> ViewportInfo {
        ViewportInfo::new(Default::default())
    }
}

impl ViewportInfo {
    /// Create a new viewport info with the viewport and screen aligned
    pub fn new(screen_size: Size) -> ViewportInfo {
        let mut viewport_rect = Rect::default();
        viewport_rect.size = screen_size.clone();

        ViewportInfo {
            viewport_rect,
            screen_size,
            viewport_scale: 1.,
        }
    }

    /// Resize the screen and adjust the viewport to match.  This method assumes
    /// that the coordinate system in the view will resize around the top-left
    /// corner of the screen when the screen is resized
    ///
    pub fn resize_screen(&mut self, new_size: Size) {
        let mut position_shift = Point::new(
            (self.screen_size.width - new_size.width) / 2.,
            (self.screen_size.height - new_size.height) / 2.,
        );

        position_shift *= self.viewport_scale;

        let new_position = Point::new(
            self.viewport_rect.top_left.x + position_shift.x,
            self.viewport_rect.top_left.y + position_shift.y,
        );

        self.screen_size = new_size;
        self.viewport_rect.size = &self.screen_size * self.viewport_scale;
        self.viewport_rect.top_left = new_position;

        debug!("Viewport info changed to {:?}", self);
    }

    /// change the scale of the area shown by the viewport by the given
    /// additive amount and center the zoom on the given point in screen
    /// coordinates
    pub fn change_scale_additive_around_center_point(
        &mut self,
        scale_change_additive: f64,
        magnify_center_screen_point: Point,
    ) {
        let new_scale = self.viewport_scale * (1. - scale_change_additive);

        self.viewport_scale = new_scale;

        let new_size = &self.screen_size * new_scale;

        let magnify_center_fraction = Point::new(
            magnify_center_screen_point.x / self.screen_size.width,
            magnify_center_screen_point.y / self.screen_size.height,
        );

        let position_shift = {
            let size = &self.viewport_rect.size;

            Point::new(
                (size.width - new_size.width) * magnify_center_fraction.x,
                (size.height - new_size.height) * magnify_center_fraction.y,
            )
        };

        let new_position = Point::new(
            self.viewport_rect.top_left.x + position_shift.x,
            self.viewport_rect.top_left.y + position_shift.y,
        );

        self.viewport_rect.size = new_size;
        self.viewport_rect.top_left = new_position;

        debug!("Viewport changed to {:?}", self);
    }

    /// change the scale of the area shown by the viewport by the given
    /// additive amount and shift the position of the viewport by the given
    /// amount
    pub fn change_scale_and_move(
        &mut self,
        scale: f64,
        mut position_shift: Point,
    ) {
        position_shift *= self.viewport_scale;

        let new_position = Point::new(
            self.viewport_rect.top_left.x + position_shift.x,
            self.viewport_rect.top_left.y + position_shift.y,
        );

        self.viewport_scale *= scale;

        let new_size = &self.screen_size * self.viewport_scale;

        self.viewport_rect.size = new_size;
        self.viewport_rect.top_left = new_position;

        debug!("Viewport changed to {:?}", self);
    }

    /// Move the viewport to the new given top-left point
    pub fn move_viewport(&mut self, new_top_left: Point) {
        self.viewport_rect.top_left = new_top_left;
        debug!("Viewport changed to {:?}", self);
    }
}
