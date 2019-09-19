
use crate::model::{ Rect, Size, Point };

#[derive(Clone, Debug)]
pub struct ViewportInfo {
    pub viewport_rect: Rect,
    pub screen_size: Size,
    pub viewport_scale: f64
}

impl ViewportInfo {

    /// Create a new viewport info with the viewport and screen aligned
    pub fn new(screen_size: Size) -> ViewportInfo {
        let mut viewport_rect = Rect::default();
        viewport_rect.size = screen_size.clone();

        ViewportInfo {
            viewport_rect: viewport_rect,
            screen_size: screen_size,
            viewport_scale: 1.
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
    /// additive amount
    pub fn change_scale_additive(&mut self, scale_change_additive: f64) {
        let new_scale = self.viewport_scale * ( 1. - scale_change_additive );

        let center_ratio = Point::new(0.5, 0.5);
        self.viewport_scale = new_scale;

        let new_size = &self.screen_size * new_scale;

        let position_shift = Point::new(
            (self.viewport_rect.size.width - new_size.width) * center_ratio.x,
            (self.viewport_rect.size.height - new_size.height) * center_ratio.y
        );

        let new_position = Point::new(
            self.viewport_rect.top_left.x + position_shift.x,
            self.viewport_rect.top_left.y + position_shift.y,
        );

        self.viewport_rect.size = new_size;
        self.viewport_rect.top_left = new_position;

        debug!("Viewport changed to {:?}", self);

    }

    /// Move the viewport to the new given top-left point
    pub fn move_viewport<'a>(&'a mut self, new_top_left: Point) -> &'a Point{
        self.viewport_rect.top_left = new_top_left;
        debug!("Viewport changed to {:?}", self);

        &self.viewport_rect.top_left
    }

}