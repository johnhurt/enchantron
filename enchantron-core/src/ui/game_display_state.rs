use crate::model::{Point, Rect, Size};
use crate::native::Texture;
use crate::ui::{DragState, SpriteSourceFn, TerrainGenerator, ViewportInfo};
use crate::view_types::ViewTypes;

pub struct GameDisplayState<T>
where
    T: ViewTypes,
{
    pub sprite_source: SpriteSourceFn<T::Sprite>,
    pub grass: T::Sprite,
    pub viewport_info: Option<ViewportInfo>,
    pub drag_state: Option<DragState>,
}

impl<T> GameDisplayState<T>
where
    T: ViewTypes,
{
    pub fn new(
        sprite_source: SpriteSourceFn<T::Sprite>,
    ) -> GameDisplayState<T> {
        GameDisplayState {
            sprite_source: sprite_source,
            grass: sprite_source(),

            viewport_info: Default::default(),

            drag_state: Default::default(),
        }
    }

    // Get a reference to the viewport rectangle
    pub fn get_viewport_rect<'a>(&'a self) -> Option<&'a Rect> {
        self.viewport_info.as_ref().map(|vpi| &vpi.viewport_rect)
    }

    // Get a reference to the top-left corner of the viewport rectangle
    pub fn get_viewport_top_left<'a>(&'a self) -> Option<&'a Point> {
        self.get_viewport_rect().map(|vpr| &vpr.top_left)
    }

    pub fn get_viewport_scale(&self) -> f64 {
        self.viewport_info
            .as_ref()
            .map(|vpi| vpi.viewport_scale)
            .unwrap_or(1.)
    }

    /// Update the layout of the display based on a change in the size of
    /// screen
    pub fn layout<'a>(&'a mut self, new_size: Size) -> &'a ViewportInfo {
        if let Some(ref mut viewport_info) = self.viewport_info {
            viewport_info.resize_screen(new_size);
        } else {
            self.viewport_info = Some(ViewportInfo::new(new_size));
        }

        self.viewport_info.as_ref().unwrap()
    }

    /// change the scale of the area shown by the viewport by the given
    /// additive amount, and return the new scale. The center of the zoom
    pub fn change_scale_additive<'a>(
        &'a mut self,
        scale_change_additive: f64,
        magnify_center_screen_point: Point,
    ) -> Option<&'a ViewportInfo> {
        if let Some(ref mut viewport_info) = self.viewport_info {
            viewport_info.change_scale_additive(
                scale_change_additive,
                magnify_center_screen_point,
            );
        } else {
            error!("No viewport rectangle found when scaling");
            panic!("No viewport rectangle found when scaling");
        }

        self.viewport_info.as_ref()
    }

    /// Move the viewport rect's top left to the given point and return a
    /// ref to the resulting top_left
    pub fn move_viewport<'a>(&'a mut self, new_top_left: Point) -> &'a Point {
        if let Some(ref mut viewport_info) = self.viewport_info {
            viewport_info.move_viewport(new_top_left)
        } else {
            error!("No viewport rectangle found when panning");
            panic!("No viewport rectangle found when panning");
        }
    }
}
