use std::iter;
use std::sync::Arc;

use crate::event::{EventBus, ListenerRegistration, ViewportChange};
use crate::game::constants;
use crate::model::{IPoint, IRect, ISize, Rect, UPoint, URect};
use crate::ui::ViewportInfo;
use crate::view_types::ViewTypes;

use super::{
    HasMutableLocation, HasMutableSize, HasMutableVisibility, HasMutableZLevel,
    Sprite, SpriteSource, SpriteSourceWrapper, TerrainTextureProvider,
    TerrainUpdateInfo,
};

use tokio::stream::StreamExt;
use tokio::sync::{Mutex, RwLock};

const UNIT_ZOOM_LEVEL_TILE_LENGTH: usize = 16;
const UNIT_ZOOM_LEVEL_SPRITE_WIDTH_IN_TILES: usize = 16;
const TERRAIN_SPRITE_TEXTURE_WIDTH: usize = 128;

/// Get the fractional zoom level for the given viewport and terrain rect
fn get_fractional_zoom_level(viewport_info: &ViewportInfo) -> f64 {
    let scale = f64::min(
        viewport_info.viewport_rect.size.width
            / viewport_info.screen_size.width as f64,
        viewport_info.viewport_rect.size.height
            / viewport_info.screen_size.height as f64,
    );

    f64::max(1., scale).log2()
}

/// Get the minimum terrain sprite rect needed to cover the given terrain rect
/// at the given zoom level as well as the size of the 2-D array of sprites
/// needed to cover it.
fn get_min_sprite_covering(
    zoom_level: usize,
    terrain_rect: &IRect,
) -> (IRect, ISize) {
    let sprite_width_in_tiles =
        (UNIT_ZOOM_LEVEL_SPRITE_WIDTH_IN_TILES * (1 << zoom_level)) as i64;

    let mut top_left = IPoint::new(
        terrain_rect.top_left.x.div_euclid(sprite_width_in_tiles),
        terrain_rect.top_left.y.div_euclid(sprite_width_in_tiles),
    );

    top_left *= sprite_width_in_tiles;

    let unrounded_size = terrain_rect.bottom_right_exclusive() - &top_left;

    let mut sprite_array_size = &unrounded_size / sprite_width_in_tiles;

    let mut rounded_size = &sprite_array_size * sprite_width_in_tiles;

    if unrounded_size.x > rounded_size.x {
        sprite_array_size.x += 1;
        rounded_size.x += sprite_width_in_tiles;
    }

    if unrounded_size.y > rounded_size.y {
        sprite_array_size.y += 1;
        rounded_size.y += sprite_width_in_tiles;
    }

    (
        IRect {
            top_left,
            size: rounded_size.to_size().unwrap(),
        },
        sprite_array_size.to_size().unwrap(),
    )
}

pub struct TerrainGenerator<T>
where
    T: ViewTypes,
{
    sprite_source: SpriteSourceWrapper<T>,
    terrain_texture_provider: TerrainTextureProvider<T>,
    listener_registrations: Mutex<Vec<ListenerRegistration>>,
    inner: RwLock<Inner<T::Sprite>>,
}

struct Inner<S>
where
    S: Sized + HasMutableSize + HasMutableLocation + HasMutableVisibility,
{
    terrain_sprites_size: ISize,
    terrain_sprites: Vec<Vec<S>>,
    sprite_terrain_coverage: IRect,
    top_left_sprite: UPoint,
    zoom_level: usize,
}

impl<T> TerrainGenerator<T>
where
    T: ViewTypes,
{
    async fn add_listener_registration(
        &self,
        listener_registration: ListenerRegistration,
    ) {
        self.listener_registrations
            .lock()
            .await
            .push(listener_registration);
    }

    pub async fn new(
        event_bus: EventBus,
        sprite_source: SpriteSourceWrapper<T>,
        terrain_texture_provider: TerrainTextureProvider<T>,
    ) -> Arc<TerrainGenerator<T>> {
        let result = Arc::new(TerrainGenerator {
            sprite_source: sprite_source,
            terrain_texture_provider: terrain_texture_provider,

            listener_registrations: Default::default(),

            inner: RwLock::new(Inner::default()),
        });

        let weak_self = Arc::downgrade(&result);

        let (listener_registration, mut event_stream) =
            event_bus.register::<ViewportChange>();

        result
            .add_listener_registration(listener_registration)
            .await;

        event_bus.spawn(async move {
            while let Some(event) = event_stream.next().await {
                if let Some(arc_self) = weak_self.upgrade() {
                    arc_self.on_viewport_change(&event.new_viewport).await
                } else {
                    break;
                }
            }
        });

        result
    }

    /// Run the given action with a read-only reference to the inner terrain
    /// generator
    async fn with_inner<R>(
        &self,
        action: impl FnOnce(&Inner<T::Sprite>) -> R,
    ) -> R {
        action(&(*self.inner.read().await))
    }

    /// Run the given action with a rw reference to the inner terrain
    /// generator
    async fn with_inner_mut<R>(
        &self,
        action: impl FnOnce(&mut Inner<T::Sprite>) -> R,
    ) -> R {
        action(&mut (*self.inner.write().await))
    }

    /// Called when the viewport changes to adjust the terrain.  This method
    /// 1. Checks to see if the terrain needs to be updated to contain the
    ///    given viewport, and if not, the method returns
    /// 2. checks to see if the size of the terrain sprites is big enough to
    ///    contain the viewport rect given
    /// 3. if the terrain sprites needs to be altered, increase the size of the
    ///    terrain sprites array and updates
    /// 4. ?
    async fn on_viewport_change(&self, viewport_info: &ViewportInfo) {
        let terrain_rect_opt = self
            .with_inner(|inner| inner.terrain_updates_required(viewport_info))
            .await;

        if terrain_rect_opt.is_none() {
            debug!("No terrain updates");
            return;
        }

        let terrain_update_info = terrain_rect_opt.unwrap();

        let valid_sprite_rect = {
            if self
                .with_inner(|inner| {
                    inner.check_sprite_array_size_increased(
                        &terrain_update_info.sprite_array_size,
                    )
                })
                .await
            {
                debug!("Size increased");
                self.with_inner_mut(|inner| {
                    inner.increase_size_for(&terrain_update_info, || {
                        let result = self.sprite_source.create_sprite();
                        result.set_z_level(constants::TERRAIN_Z_LEVEL);
                        result
                    })
                })
                .await
            } else {
                debug!("Size not increased");
                let (top_left, new_valid_rect) = self
                    .with_inner(|inner| {
                        inner.calculate_new_valid_sprites(&terrain_update_info)
                    })
                    .await
                    .unwrap_or_default();

                self.with_inner_mut(|inner| {
                    inner.update_terrain_sprites_location(
                        terrain_update_info.terrain_rect.clone(),
                        top_left,
                    );
                })
                .await;

                new_valid_rect
            }
        };

        let texture_width = UNIT_ZOOM_LEVEL_TILE_LENGTH
            * terrain_update_info.sprite_length_in_tiles;
        let texture_size = ISize::new(
            TERRAIN_SPRITE_TEXTURE_WIDTH,
            TERRAIN_SPRITE_TEXTURE_WIDTH,
        );

        self.with_inner(|inner| {
            inner.update_terrain_sprites(valid_sprite_rect, |sprite, point| {
                let texture_terrain_rect = IRect {
                    top_left: point.clone(),
                    size: ISize::new(
                        terrain_update_info.sprite_length_in_tiles,
                        terrain_update_info.sprite_length_in_tiles,
                    ),
                };

                info!("sprite rect: {:?}", texture_terrain_rect);
                info!("sprite size: {:?}", texture_size);

                sprite.set_texture(
                    &self.terrain_texture_provider.get_texture_for_rect(
                        &texture_terrain_rect,
                        &texture_size,
                    ),
                );

                sprite.set_size(texture_width as f64, texture_width as f64);
            });
        })
        .await;
    }
}

impl<T> Default for Inner<T>
where
    T: Sized + HasMutableLocation + HasMutableSize + HasMutableVisibility,
{
    fn default() -> Inner<T> {
        Inner::new()
    }
}

impl<T> Inner<T>
where
    T: Sized + HasMutableLocation + HasMutableSize + HasMutableVisibility,
{
    pub fn new() -> Inner<T> {
        Inner {
            terrain_sprites: Default::default(),
            terrain_sprites_size: Default::default(),
            sprite_terrain_coverage: Default::default(),
            top_left_sprite: Default::default(),
            zoom_level: Default::default(),
        }
    }

    /// Get the terrain rect required to cover the given viewport rect based on
    /// the current size of the terrain sprites array.
    fn viewport_rect_to_terrain_rect(&self, viewport_rect: &Rect) -> IRect {
        let tile_size_f64 = UNIT_ZOOM_LEVEL_TILE_LENGTH as f64;

        let viewport_top_left = &viewport_rect.top_left;
        let viewport_bottom_right = viewport_top_left + &viewport_rect.size;

        let top_left = IPoint {
            x: (viewport_top_left.x / tile_size_f64).floor() as i64,
            y: (viewport_top_left.y / tile_size_f64).floor() as i64,
        };

        let bottom_right = IPoint {
            x: (viewport_bottom_right.x / tile_size_f64).ceil() as i64,
            y: (viewport_bottom_right.y / tile_size_f64).ceil() as i64,
        };

        let size = (bottom_right - &top_left).to_size().expect("bad size");

        IRect {
            top_left: top_left,
            size: size,
        }
    }

    /// Determine if the terrain sprites need updating, and return the new terrain
    /// rect, if an update is needed
    fn terrain_updates_required(
        &self,
        viewport_info: &ViewportInfo,
    ) -> Option<TerrainUpdateInfo> {
        let ref viewport_rect = viewport_info.viewport_rect;

        let curr_zoom_level = self.zoom_level;
        let curr_zoom_level_f64 = curr_zoom_level as f64;
        let max_fractional_zoom_level = curr_zoom_level_f64 + 0.6;
        let min_fractional_zoom_level = curr_zoom_level_f64 - 0.6;

        let mut terrain_rect =
            self.viewport_rect_to_terrain_rect(viewport_rect);

        let new_fractional_zoom = get_fractional_zoom_level(viewport_info);

        let new_zoom_level = if new_fractional_zoom > max_fractional_zoom_level
            || new_fractional_zoom < min_fractional_zoom_level
        {
            new_fractional_zoom.round() as usize
        } else {
            curr_zoom_level
        };

        if new_zoom_level == curr_zoom_level
            && self.sprite_terrain_coverage.contains_rect(&terrain_rect)
        {
            return None;
        }

        let (min_covered_terrain, mut min_sprite_array_size) =
            get_min_sprite_covering(new_zoom_level, &terrain_rect);

        let tiles_per_sprite =
            UNIT_ZOOM_LEVEL_SPRITE_WIDTH_IN_TILES * (1 << new_zoom_level);
        let tiles_per_sprite_i64 = tiles_per_sprite as i64;

        // If the minimum size of the sprite array is less than the current
        // size of the sprite array in either dimension, center the minimum
        // terrain rect within a larger terrain rect that has the same size as
        // what can be covered by the current sprite array
        if min_sprite_array_size.width <= self.terrain_sprites_size.width {
            let dw =
                self.terrain_sprites_size.width - min_sprite_array_size.width;
            terrain_rect.top_left.x -= (dw as i64 / 2) * tiles_per_sprite_i64;
            terrain_rect.size.width =
                self.terrain_sprites_size.width * tiles_per_sprite;
            min_sprite_array_size.width = self.terrain_sprites_size.width;
        }

        if min_sprite_array_size.height <= self.terrain_sprites_size.height {
            let dh =
                self.terrain_sprites_size.height - min_sprite_array_size.height;
            terrain_rect.top_left.y -= (dh as i64 / 2) * tiles_per_sprite_i64;
            terrain_rect.size.height =
                self.terrain_sprites_size.height * tiles_per_sprite;
            min_sprite_array_size.height = self.terrain_sprites_size.height;
        }

        Some(TerrainUpdateInfo {
            zoom_level: new_zoom_level,
            terrain_rect: min_covered_terrain,
            sprite_length_in_tiles: tiles_per_sprite,
            sprite_array_size: min_sprite_array_size,
        })
    }

    /// Add the two points together whithin the bounds of the sprite grid system
    fn sprite_grid_add(&self, lhs: &UPoint, rhs: &IPoint) -> UPoint {
        let sprite_grid_width = self.terrain_sprites_size.width as i64;
        let sprite_grid_height = self.terrain_sprites_size.height as i64;

        UPoint::new(
            (lhs.x as i64 + rhs.x).rem_euclid(sprite_grid_width) as usize,
            (lhs.y as i64 + rhs.y).rem_euclid(sprite_grid_height) as usize,
        )
    }

    /// Assume that the given point p is a point within the bounds of the
    /// sprite grid, get the offset from origin of the sprite grid (the top left)
    fn sprite_grid_offset_from_origin(&self, p: &UPoint) -> IPoint {
        let grid_width = self.terrain_sprites_size.width as i64;
        let grid_height = self.terrain_sprites_size.height as i64;

        IPoint::new(
            (p.x as i64 - self.top_left_sprite.x as i64).rem_euclid(grid_width),
            (p.y as i64 - self.top_left_sprite.y as i64)
                .rem_euclid(grid_height),
        )
    }

    // Subtract one grid point from another within the bounds of the grid
    fn sprite_grid_sub(&self, lhs: &UPoint, rhs: &UPoint) -> IPoint {
        &self.sprite_grid_offset_from_origin(lhs)
            - &self.sprite_grid_offset_from_origin(rhs)
    }

    /// Determine which portion if any of the existing valid terrain sprites will
    /// remain valid when the viewport shifts to the given terrain rect, and
    /// return the new top-left point within the sprite array, and the region of
    /// the sprite array that's still valid
    fn calculate_new_valid_sprites(
        &self,
        terrain_update_info: &TerrainUpdateInfo,
    ) -> Option<(UPoint, URect)> {
        // A change in zoom level means that none of the sprites are valid
        if terrain_update_info.zoom_level != self.zoom_level {
            return None;
        }

        let ref new_terrain_rect = terrain_update_info.terrain_rect;
        let tiles_per_sprite = terrain_update_info.sprite_length_in_tiles;

        self.sprite_terrain_coverage
            .intersection(new_terrain_rect)
            .map(|itx_rect| {
                // Determine the top left and size of the valid region
                let mut valid_top_left_shift =
                    &itx_rect.top_left - &self.sprite_terrain_coverage.top_left;

                valid_top_left_shift /= tiles_per_sprite as i64;

                let valid_sprites_top_left = self.sprite_grid_add(
                    &self.top_left_sprite,
                    &valid_top_left_shift,
                );

                let new_valid_rect = URect {
                    top_left: valid_sprites_top_left,
                    size: &itx_rect.size / tiles_per_sprite,
                };

                // determine the new top left of the viewport rect

                let mut new_top_left_shift = &new_terrain_rect.top_left
                    - &self.sprite_terrain_coverage.top_left;

                new_top_left_shift /= tiles_per_sprite as i64;

                let new_top_left = self.sprite_grid_add(
                    &self.top_left_sprite,
                    &new_top_left_shift,
                );

                debug!("valid sprite area = {}", new_valid_rect.area());

                (new_top_left, new_valid_rect)
            })
    }

    /// Get the sprite at the given point in the sprite grid using the given natural
    /// origin.  Also return the terrain coordinates of the sprite based on the
    /// configured top-left sprite in the grid and the sprite-terrain coverage
    fn get_sprite_at<'a>(
        &'a self,
        natural_origin: &UPoint,
        natural_x: &usize,
        natural_y: &usize,
    ) -> (&'a T, IPoint) {
        let real_point = UPoint::new(
            (natural_origin.x + natural_x) % self.terrain_sprites_size.width,
            (natural_origin.y + natural_y) % self.terrain_sprites_size.height,
        );

        let sprite = self.get_sprite(&real_point).unwrap_or_else(|| {
            error!("Invalid sprite_coordinate {:?}", real_point);
            panic!("Index out of bounds error in terrain sprites array");
        });

        let offset = self.sprite_grid_offset_from_origin(&real_point)
            * (UNIT_ZOOM_LEVEL_SPRITE_WIDTH_IN_TILES << self.zoom_level) as i64;

        let terrain_point = &self.sprite_terrain_coverage.top_left + &offset;

        (sprite, terrain_point)
    }

    /// Update the invalid terrain sprites
    fn update_terrain_sprites(
        &self,
        new_valid_rect: URect,
        sprite_updater: impl Fn(&T, &IPoint),
    ) {
        debug!("Updating terrain sprites");

        // hit all the partial rows to the right of the valid region

        let new_valid_size = &new_valid_rect.size;
        let valid_top_left = &new_valid_rect.top_left;

        let action = |x: usize, y: usize| {
            let (sprite, terrain_point) =
                self.get_sprite_at(valid_top_left, &x, &y);
            sprite.set_location_point(
                &(&terrain_point * UNIT_ZOOM_LEVEL_TILE_LENGTH as f64),
            );
            sprite_updater(sprite, &terrain_point);
        };

        for y in 0..new_valid_size.height {
            for x in new_valid_size.width..self.terrain_sprites_size.width {
                action(x, y);
            }
        }

        // hit all the complete rows below the valid region

        for y in new_valid_size.height..self.terrain_sprites_size.height {
            for x in 0..self.terrain_sprites_size.width {
                action(x, y);
            }
        }
    }

    /// return true if the current size of the 2d vector array is bigger than
    /// or equal to the size given in both height and width
    fn check_sprite_array_size_increased(
        &self,
        min_sprite_array_size: &ISize,
    ) -> bool {
        min_sprite_array_size.width > self.terrain_sprites_size.width
            || min_sprite_array_size.height > self.terrain_sprites_size.height
    }

    /// Update the information about the terrain sprites that describes which part
    /// of the terrain is being shown and where in the sprite array the top-left
    /// corner is
    fn update_terrain_sprites_location(
        &mut self,
        new_terrain_rect: IRect,
        top_left_sprite: UPoint,
    ) {
        self.top_left_sprite = top_left_sprite;
        self.sprite_terrain_coverage = new_terrain_rect;
    }

    /// Increase the size of the 2d array of terrain sprites to accomodate the
    /// given size
    fn increase_size_for(
        &mut self,
        terrain_update_info: &TerrainUpdateInfo,
        sprite_source: impl Fn() -> T,
    ) -> URect {
        let ref min_sprite_array_size = terrain_update_info.sprite_array_size;

        if !self.check_sprite_array_size_increased(min_sprite_array_size) {
            // ^ double checked lock
            debug!("Double checked lock tripped");
            return URect {
                top_left: self.top_left_sprite.clone(),
                size: self.terrain_sprites_size.clone(),
            };
        }

        let tiles_per_sprite = terrain_update_info.sprite_length_in_tiles;
        let ref terrain_rect = terrain_update_info.terrain_rect;

        debug!(
            "Increasing terrain sprites cache to {:?}",
            min_sprite_array_size
        );

        let width_inc = min_sprite_array_size
            .width
            .checked_sub(self.terrain_sprites_size.width)
            .unwrap_or_default();
        let height_inc = min_sprite_array_size
            .height
            .checked_sub(self.terrain_sprites_size.height)
            .unwrap_or_default();

        let sprite_size_f64 =
            (UNIT_ZOOM_LEVEL_TILE_LENGTH * tiles_per_sprite) as f64;

        let sprite_source_with_vis_and_size = || {
            let result = sprite_source();
            result.set_size(sprite_size_f64, sprite_size_f64);
            result.set_visible(true);
            result
        };

        if width_inc > 0 {
            self.increase_row_width_by(
                width_inc,
                &sprite_source_with_vis_and_size,
            );
        }

        if height_inc > 0 {
            self.increase_row_count_by(
                height_inc,
                &sprite_source_with_vis_and_size,
            );
        }

        let (new_top_left, valid_rect) = self
            .calculate_new_valid_sprites(terrain_update_info)
            .unwrap_or_default();

        self.update_terrain_sprites_location(
            terrain_rect.clone(),
            new_top_left,
        );

        valid_rect
    }

    /// Icnrease the size of all the existing rows in the terrain to the given
    /// width
    fn increase_row_width_by(
        &mut self,
        cols_to_add: usize,
        sprite_source: &impl Fn() -> T,
    ) {
        let insert_point = &self.top_left_sprite.x;

        self.terrain_sprites.iter_mut().for_each(|row| {
            let to_insert = iter::repeat_with(sprite_source).take(cols_to_add);

            row.splice(insert_point..insert_point, to_insert)
                .for_each(|_| {});
        });

        self.top_left_sprite.x += cols_to_add;
        self.terrain_sprites_size.width += cols_to_add;
    }

    /// Increase the number of rows in the sprite array by the given number
    fn increase_row_count_by(
        &mut self,
        rows_to_add: usize,
        sprite_source: &impl Fn() -> T,
    ) {
        let total_columns = self.terrain_sprites_size.width;

        let new_rows = iter::repeat_with(|| {
            iter::repeat_with(sprite_source)
                .take(total_columns)
                .collect()
        })
        .take(rows_to_add);

        let insert_point = &self.top_left_sprite.y;

        self.terrain_sprites
            .splice(insert_point..insert_point, new_rows);

        self.top_left_sprite.y += rows_to_add;
        self.terrain_sprites_size.height += rows_to_add;
    }

    fn get_sprite<'a>(&'a self, point: &UPoint) -> Option<&'a T> {
        self.terrain_sprites
            .get(point.y)
            .and_then(|row| row.get(point.x))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Point, Size};
    use atomic_counter::*;
    use std::cell::RefCell;

    #[derive(Default)]
    struct TestTile {
        size: RefCell<Size>,
        location: RefCell<Point>,
        visible: RefCell<bool>,

        updated: ConsistentCounter,
    }

    impl HasMutableLocation for TestTile {
        fn set_location_animated(&self, left: f64, top: f64, _: f64) {
            let mut loc = self.location.borrow_mut();
            loc.x = left;
            loc.y = top;
        }
    }

    impl HasMutableSize for TestTile {
        fn set_size_animated(&self, width: f64, height: f64, _: f64) {
            let mut size = self.size.borrow_mut();
            size.height = height;
            size.width = width;
        }
    }

    impl HasMutableVisibility for TestTile {
        fn set_visible(&self, visible: bool) {
            let mut vis = self.visible.borrow_mut();
            *vis = visible;
        }
    }

    fn default_test_terrain_generator() -> Inner<TestTile> {
        Inner::new()
    }

    fn default_viewport() -> ViewportInfo {
        ViewportInfo::new(Size::new(770., 1334.))
    }

    #[test]
    fn test_terrain_updates_required() {
        let mut this = default_test_terrain_generator();

        let tile_size_f64 = UNIT_ZOOM_LEVEL_TILE_LENGTH as f64;

        let mut new_viewport = ViewportInfo::new(Size::new(
            2. * tile_size_f64,
            3. * tile_size_f64,
        ));

        new_viewport.viewport_rect.size.width = 2. * tile_size_f64;
        new_viewport.viewport_rect.size.height = 3. * tile_size_f64;
        new_viewport.viewport_rect.top_left.x = 0.5 * tile_size_f64;
        new_viewport.viewport_rect.top_left.y = -1.5 * tile_size_f64;

        let terrain_update_info_opt =
            this.terrain_updates_required(&new_viewport);

        assert!(terrain_update_info_opt.is_some());

        let terrain_update_info = terrain_update_info_opt.unwrap();

        assert_eq!(terrain_update_info.zoom_level, 0);
        assert_eq!(
            terrain_update_info.terrain_rect,
            IRect::new(0, -16, 16, 32)
        );

        new_viewport.viewport_rect.top_left.x = 0.1 * tile_size_f64;
        new_viewport.viewport_rect.top_left.y = 0.9 * tile_size_f64;
        new_viewport.viewport_rect.size.width = 1.91 * tile_size_f64;
        new_viewport.viewport_rect.size.height = 3. * tile_size_f64;

        let terrain_update_info_opt =
            this.terrain_updates_required(&new_viewport);

        assert!(terrain_update_info_opt.is_some());

        let terrain_update_info = terrain_update_info_opt.unwrap();

        assert_eq!(terrain_update_info.zoom_level, 0);
        assert_eq!(terrain_update_info.terrain_rect, IRect::new(0, 0, 16, 16));

        // verify the centering of smaller viewports in larger sprite buffers
        this = default_test_terrain_generator();

        new_viewport.viewport_rect.top_left.x = -2. * tile_size_f64;
        new_viewport.viewport_rect.top_left.y = -2. * tile_size_f64;
        new_viewport.viewport_rect.size.width = 4. * tile_size_f64;
        new_viewport.viewport_rect.size.height = 4. * tile_size_f64;

        let terrain_update_info_opt =
            this.terrain_updates_required(&new_viewport);

        assert!(terrain_update_info_opt.is_some());

        let terrain_update_info = terrain_update_info_opt.unwrap();

        assert_eq!(terrain_update_info.zoom_level, 0);
        assert_eq!(
            terrain_update_info.terrain_rect,
            IRect::new(-16, -16, 32, 32)
        );

        this.increase_size_for(&terrain_update_info, Default::default);

        new_viewport.viewport_rect.top_left.x = -2. * tile_size_f64;
        new_viewport.viewport_rect.top_left.y = -2. * tile_size_f64;
        new_viewport.viewport_rect.size.width = 2. * tile_size_f64;
        new_viewport.viewport_rect.size.height = 2. * tile_size_f64;

        let terrain_update_info_opt =
            this.terrain_updates_required(&new_viewport);

        assert_eq!(terrain_update_info_opt, None);

        new_viewport.viewport_rect.top_left.x = 15. * tile_size_f64;
        new_viewport.viewport_rect.top_left.y = 15. * tile_size_f64;
        new_viewport.viewport_rect.size.width = 2. * tile_size_f64;
        new_viewport.viewport_rect.size.height = 2. * tile_size_f64;

        let terrain_update_info_opt =
            this.terrain_updates_required(&new_viewport);

        assert!(terrain_update_info_opt.is_some());

        let terrain_update_info = terrain_update_info_opt.unwrap();

        assert_eq!(terrain_update_info.zoom_level, 0);
        assert_eq!(terrain_update_info.terrain_rect, IRect::new(0, 0, 32, 32));
    }

    #[test]
    fn test_size_increased() {
        let mut this = default_test_terrain_generator();

        let mut viewport_info = default_viewport();

        let sprite_size_f64 = UNIT_ZOOM_LEVEL_TILE_LENGTH as f64;

        viewport_info.viewport_rect.top_left.x = -2. * sprite_size_f64;
        viewport_info.viewport_rect.top_left.y = -2. * sprite_size_f64;
        viewport_info.viewport_rect.size.width = 4. * sprite_size_f64;
        viewport_info.viewport_rect.size.height = 4. * sprite_size_f64;

        let terrain_update_info_opt =
            this.terrain_updates_required(&viewport_info);

        assert!(terrain_update_info_opt.is_some());

        let terrain_update_info = terrain_update_info_opt.unwrap();

        assert_eq!(terrain_update_info.zoom_level, 0);
        assert_eq!(
            terrain_update_info.terrain_rect,
            IRect::new(-16, -16, 32, 32)
        );

        assert!(this.check_sprite_array_size_increased(
            &terrain_update_info.sprite_array_size
        ));

        this.increase_size_for(&terrain_update_info, Default::default);

        assert_eq!(this.sprite_terrain_coverage, IRect::new(-16, -16, 32, 32));
    }

    #[test]
    fn test_calculate_vew_valid_rect() {
        let mut this = default_test_terrain_generator();

        let mut viewport_info = default_viewport();

        let sprite_size_f64 = UNIT_ZOOM_LEVEL_TILE_LENGTH as f64;

        viewport_info.viewport_rect.top_left.x = -2. * sprite_size_f64;
        viewport_info.viewport_rect.top_left.y = -2. * sprite_size_f64;
        viewport_info.viewport_rect.size.width = 4. * sprite_size_f64;
        viewport_info.viewport_rect.size.height = 4. * sprite_size_f64;

        let terrain_update_info_opt =
            this.terrain_updates_required(&viewport_info);

        assert!(terrain_update_info_opt.is_some());

        let mut terrain_update_info = terrain_update_info_opt.unwrap();

        assert_eq!(terrain_update_info.zoom_level, 0);
        assert_eq!(
            terrain_update_info.terrain_rect,
            IRect::new(-16, -16, 32, 32)
        );

        assert!(this.check_sprite_array_size_increased(
            &terrain_update_info.sprite_array_size
        ));

        this.increase_size_for(&terrain_update_info, Default::default);

        assert_eq!(this.sprite_terrain_coverage, IRect::new(-16, -16, 32, 32));

        assert_eq!(
            this.calculate_new_valid_sprites(&terrain_update_info),
            Some((UPoint::default(), URect::new(0, 0, 2, 2)))
        );

        terrain_update_info.terrain_rect = IRect::new(16, 16, 32, 32);

        assert_eq!(
            this.calculate_new_valid_sprites(&terrain_update_info),
            None
        );

        terrain_update_info.terrain_rect = IRect::new(0, 0, 32, 32);

        assert_eq!(
            this.calculate_new_valid_sprites(&terrain_update_info),
            Some((UPoint::new(1, 1), URect::new(1, 1, 1, 1)))
        );

        terrain_update_info.terrain_rect = IRect::new(-32, -32, 32, 32);

        let (new_top_left, new_valid_rect) = this
            .calculate_new_valid_sprites(&terrain_update_info)
            .unwrap_or_default();

        assert_eq!(UPoint::new(1, 1), new_top_left);
        assert_eq!(URect::new(0, 0, 1, 1), new_valid_rect);

        this.update_terrain_sprites_location(
            IRect::new(-32, -32, 32, 32),
            new_top_left,
        );

        this.update_terrain_sprites(new_valid_rect, |sprite, _| {
            sprite.updated.inc();
        });

        assert_eq!(
            this.get_sprite(&UPoint::new(0, 0)).map(|t| t.updated.get()),
            Some(0)
        );
        assert_eq!(
            this.get_sprite(&UPoint::new(0, 1)).map(|t| t.updated.get()),
            Some(1)
        );
        assert_eq!(
            this.get_sprite(&UPoint::new(1, 0)).map(|t| t.updated.get()),
            Some(1)
        );
        assert_eq!(
            this.get_sprite(&UPoint::new(1, 1)).map(|t| t.updated.get()),
            Some(1)
        );

        assert_eq!(
            this.get_sprite(&UPoint::new(1, 1))
                .map(|t| t.location.borrow().clone()),
            Some(Point::new(-512., -512.))
        );
    }

    #[test]
    fn test_get_sprite_size() {
        let mut viewport_info = ViewportInfo::new(Size::new(750., 1334.));
        viewport_info.viewport_rect = Rect::new(0., 0., 750., 1334.);

        let mut zoom_level = get_fractional_zoom_level(&viewport_info);
        assert_eq!(0., zoom_level);

        viewport_info.viewport_rect = Rect::new(0., 0., 750. * 2., 1334. * 2.);

        zoom_level = get_fractional_zoom_level(&viewport_info);
        assert_eq!(1., zoom_level);
    }
}
