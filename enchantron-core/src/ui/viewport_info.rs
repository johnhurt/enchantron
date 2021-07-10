use crate::game::constants;
use crate::model::{IPoint, Point, Rect, Size};

/// Get the size of the viewport based on the given screen size, screen scale,
/// and viewport scale. The formula for this is
///
/// viewport_size = U * screen_size / screen_scale * viewport_scale
///
/// Where U is size of one terrain unit in the number of scaled pixels when
/// viewport_scale = 1.  This value is defined in game::constants
fn get_viewport_size(
    screen_size: Size,
    screen_scale: f64,
    viewport_scale: f64,
) -> Size {
    screen_size * (viewport_scale / screen_scale)
}

#[derive(Clone, Debug, Copy)]
pub struct ViewportInfo {
    pub viewport_rect: Rect,
    pub viewport_scale: f64,
    pub screen_size: Size,
    pub screen_scale: f64,
}

impl Default for ViewportInfo {
    fn default() -> ViewportInfo {
        ViewportInfo::new(Default::default(), 1.0)
    }
}

impl ViewportInfo {
    /// Create a new viewport info with the viewport and screen aligned
    pub fn new(screen_size: Size, screen_scale: f64) -> ViewportInfo {
        let mut viewport_rect = Rect::default();
        viewport_rect.size = get_viewport_size(screen_size, screen_scale, 1.0);

        ViewportInfo {
            viewport_rect,
            viewport_scale: 1.,
            screen_size,
            screen_scale,
        }
    }

    /// Resize the screen and adjust the viewport to match.  This method assumes
    /// that the coordinate system in the view will resize around the top-left
    /// corner of the screen when the screen is resized
    ///
    pub fn resize_screen(&mut self, new_size: Size, new_screen_scale: f64) {
        let mut position_shift = (self.screen_size.as_point()
            * (1. / self.screen_scale))
            - (new_size.as_point() * (1. / new_screen_scale));

        position_shift *= self.viewport_scale;

        let new_position = Point::new(
            self.viewport_rect.top_left.x + position_shift.x,
            self.viewport_rect.top_left.y + position_shift.y,
        );

        self.screen_size = new_size;
        self.screen_scale = new_screen_scale;
        self.viewport_rect.size =
            get_viewport_size(new_size, new_screen_scale, self.viewport_scale);
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

        trace!("Viewport changed to {:?}", self);
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

        trace!("Viewport changed to {:?}", self);
    }

    /// Move the viewport to the new given top-left point
    pub fn move_viewport(&mut self, new_top_left: Point) {
        self.viewport_rect.top_left = new_top_left;
        trace!("Viewport changed to {:?}", self);
    }

    /// Get the terrain tile point for the given screen point
    pub fn get_terrain_tile_for(&self, screen_point: &Point) -> IPoint {
        ((screen_point * self.viewport_scale * (1. / 16.))
            + self.viewport_rect.top_left)
            .floor()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_viewport_scale() {
        assert_eq!(
            get_viewport_size(Size::new(600., 400.), 2.0, 1.0),
            Size::new(300., 200.)
        );

        assert_eq!(
            get_viewport_size(Size::new(600., 400.), 2.0, 4.0),
            Size::new(1200., 800.)
        );
    }
}
