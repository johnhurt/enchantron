use std::sync::Arc;

use crate::event::EventBus;
use crate::model::{Point, Rect, Size};
use crate::native::{RuntimeResources, SystemView};
use crate::ui::{
    DragTracker, SpriteSource, TerrainGenerator, TerrainTextureProvider,
    ViewportInfo,
};
use crate::view_types::ViewTypes;

pub struct GameDisplayState<T>
where
    T: ViewTypes,
{
    pub viewport_info: ViewportInfo,
    pub drag_tracker: DragTracker,
    pub terrain_generator: Arc<TerrainGenerator<T>>,

    pub character: Option<T::Sprite>,
}

impl<T> GameDisplayState<T>
where
    T: ViewTypes,
{
    pub async fn new(
        event_bus: EventBus,
        sprite_source: &impl SpriteSource<
            T = T::Texture,
            S = T::Sprite,
            G = T::SpriteGroup,
        >,
        runtime_resources: Arc<RuntimeResources<T::SystemView>>,
        system_view: Arc<T::SystemView>,
    ) -> GameDisplayState<T> {
        GameDisplayState {
            viewport_info: Default::default(),

            drag_tracker: Default::default(),
            terrain_generator: TerrainGenerator::new(
                event_bus,
                sprite_source,
                TerrainTextureProvider::new(
                    runtime_resources,
                    system_view.get_texture_loader(),
                ),
            )
            .await,

            character: None,
        }
    }

    pub fn set_character_sprite(&mut self, sprite: T::Sprite) {
        self.character = Some(sprite)
    }

    pub fn get_character_sprite<'a>(&'a self) -> Option<&'a T::Sprite> {
        self.character.as_ref()
    }

    // Get a reference to the viewport rectangle
    pub fn get_viewport_rect<'a>(&'a self) -> &'a Rect {
        &self.viewport_info.viewport_rect
    }

    // Get a reference to the top-left corner of the viewport rectangle
    pub fn get_viewport_top_left<'a>(&'a self) -> &'a Point {
        &self.get_viewport_rect().top_left
    }

    pub fn get_viewport_scale(&self) -> f64 {
        self.viewport_info.viewport_scale
    }

    /// Update the layout of the display based on a change in the size of
    /// screen
    pub fn layout<'a>(&'a mut self, new_size: Size) -> &'a ViewportInfo {
        self.viewport_info.resize_screen(new_size);
        &self.viewport_info
    }

    /// change the scale of the area shown by the viewport by the given
    /// additive amount, and return the new scale. The center of the zoom
    pub fn change_scale_additive_around_centerpoint<'a>(
        &'a mut self,
        scale_change_additive: f64,
        magnify_center_screen_point: Point,
    ) -> &'a ViewportInfo {
        self.viewport_info.change_scale_additive_around_centerpoint(
            scale_change_additive,
            magnify_center_screen_point,
        );

        &self.viewport_info
    }

    /// change the scale of the area shown by the viewport by the given
    /// additive amount, move by the given amount, and return the new scale.
    pub fn change_scale_and_move<'a>(
        &'a mut self,
        scale: f64,
        position_shift: Point,
    ) -> &'a ViewportInfo {
        self.viewport_info
            .change_scale_and_move(scale, position_shift);

        &self.viewport_info
    }

    /// Move the viewport rect's top left to the given point and return a
    /// ref to the resulting top_left
    pub fn move_viewport<'a>(
        &'a mut self,
        new_top_left: Point,
    ) -> &'a ViewportInfo {
        self.viewport_info.move_viewport(new_top_left);

        &self.viewport_info
    }

    pub fn move_viewport_by<'a>(
        &'a mut self,
        delta_top_left: Point,
    ) -> &'a ViewportInfo {
        let new_top_left =
            &self.viewport_info.viewport_rect.top_left + delta_top_left;

        self.viewport_info.move_viewport(new_top_left);

        &self.viewport_info
    }
}
