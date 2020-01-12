use std::sync::{Arc, Weak};

use crate::event::{EnchantronEvent, EventBus};
use crate::model::{Point, Rect, Size};
use crate::native::{RuntimeResources, Texture};
use crate::ui::{
    DragState, SpriteSource, SpriteSourceWrapper, TerrainGenerator,
    TerrainTextureProvider, ViewportInfo,
};
use crate::view_types::ViewTypes;

pub struct GameDisplayState<T>
where
    T: ViewTypes,
{
    pub sprite_source: SpriteSourceWrapper<T>,
    pub viewport_info: ViewportInfo,
    pub drag_state: Option<DragState>,
    pub terrain_generator: Arc<TerrainGenerator<T>>,

    pub character: Option<T::Sprite>,
}

impl<T> GameDisplayState<T>
where
    T: ViewTypes,
{
    pub async fn new(
        event_bus: EventBus,
        sprite_source: SpriteSourceWrapper<T>,
        runtime_resources: Arc<RuntimeResources<T::SystemView>>,
    ) -> GameDisplayState<T> {
        GameDisplayState {
            sprite_source: sprite_source.clone(),

            viewport_info: Default::default(),

            drag_state: Default::default(),
            terrain_generator: TerrainGenerator::new(
                event_bus,
                sprite_source,
                TerrainTextureProvider::new(runtime_resources),
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
    pub fn change_scale_additive<'a>(
        &'a mut self,
        scale_change_additive: f64,
        magnify_center_screen_point: Point,
    ) -> &'a ViewportInfo {
        self.viewport_info.change_scale_additive(
            scale_change_additive,
            magnify_center_screen_point,
        );

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
