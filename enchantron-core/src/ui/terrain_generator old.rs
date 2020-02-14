use std::iter;
use std::sync::Arc;

use crate::event::{EventBus, ListenerRegistration, ViewportChange};
use crate::game::constants;
use crate::model::{IPoint, IRect, ISize, Point, Rect, Size, UPoint, URect};
use crate::native::RuntimeResources;
use crate::view_types::ViewTypes;

use super::{
    HasMutableLocation, HasMutableSize, HasMutableVisibility, HasMutableZLevel,
    Sprite, SpriteSource, SpriteSourceWrapper, TerrainTextureProvider,
};

use tokio::stream::StreamExt;
use tokio::sync::{Mutex, RwLock};

pub const DEFAULT_TILE_SIZE: usize = 32;
pub const DEFAULT_MARGIN_FRACTION: f64 = 0.1;

pub struct TerrainGenerator<T>
where
    T: ViewTypes,
{
    sprite_source: SpriteSourceWrapper<T>,
    terrain_texture_provider: TerrainTextureProvider<T>,
    listener_registrations: Mutex<Vec<ListenerRegistration>>,
    inner: RwLock<Inner<T::Sprite>>,
}

struct Inner<T>
where
    T: Sized + HasMutableSize + HasMutableLocation + HasMutableVisibility,
{
    terrain_tiles_size: ISize,
    terrain_tiles: Vec<Vec<T>>,
    tile_size: usize,
    margin_fraction: f64,
    tile_terrain_coverage: IRect,
    top_left_tile: UPoint,
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
                    arc_self.on_viewport_change(&event.new_viewport_rect).await
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
    /// 2. checks to see if the size of the terrain tiles is big enough to
    ///    contain the viewport rect given
    /// 3. if the terrain tiles needs to be altered, increase the size of the
    ///    terrain tiles array and updates
    /// 4. ?
    async fn on_viewport_change(&self, viewport_rect: &Rect) {
        let terrain_rect_opt = self
            .with_inner(|inner| inner.terrain_updates_required(viewport_rect))
            .await;

        if terrain_rect_opt.is_none() {
            debug!("No terrain updates");
            return;
        }

        let terrain_rect = terrain_rect_opt.unwrap();

        let (valid_tile_rect, valied_terrain_rect) = {
            let min_size = &terrain_rect.size;

            if self
                .with_inner(|inner| inner.check_size_increased(min_size))
                .await
            {
                debug!("Size increased");
                self.with_inner_mut(|inner| {
                    inner.increase_size_for(terrain_rect, || {
                        let result = self.sprite_source.create_sprite();
                        result.set_z_level(constants::TERRAIN_Z_LEVEL);
                        result
                    })
                })
                .await
            } else {
                debug!("Size not increased");
                let (top_left, new_valid_rect, valid_terrain_rect) = self
                    .with_inner(|inner| {
                        inner.calculate_new_valid_tiles(&terrain_rect)
                    })
                    .await
                    .unwrap_or_default();

                self.with_inner_mut(|inner| {
                    inner.update_terrain_tiles_location(terrain_rect, top_left);
                })
                .await;

                (new_valid_rect, valid_terrain_rect)
            }
        };

        self.with_inner(|inner| {
            inner.update_terrain_tiles(valid_tile_rect, |tile, point| {
                tile.set_texture(
                    self.terrain_texture_provider.get_texture_at(point),
                );
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
        Inner::new(DEFAULT_TILE_SIZE, DEFAULT_MARGIN_FRACTION)
    }
}

impl<T> Inner<T>
where
    T: Sized + HasMutableLocation + HasMutableSize + HasMutableVisibility,
{
    pub fn new(tile_size: usize, margin_fraction: f64) -> Inner<T> {
        Inner {
            terrain_tiles: Default::default(),
            terrain_tiles_size: Default::default(),
            tile_size: tile_size,
            margin_fraction: margin_fraction,
            tile_terrain_coverage: Default::default(),
            top_left_tile: Default::default(),
        }
    }

    /// Get the terrain rect required to cover the given viewport rect based on
    /// the current size of the terrain tiles array.
    fn viewport_rect_to_terrain_rect(&self, viewport_rect: &Rect) -> IRect {
        let tile_size_f64 = self.tile_size as f64;

        let margin = self.margin_fraction
            * f64::max(viewport_rect.size.height, viewport_rect.size.width);

        let viewport_top_left = &viewport_rect.top_left;
        let viewport_bottom_right = viewport_top_left + &viewport_rect.size;

        let top_left = IPoint {
            x: ((viewport_top_left.x - margin) / tile_size_f64).floor() as i64,
            y: ((viewport_top_left.y - margin) / tile_size_f64).floor() as i64,
        };

        let bottom_right = IPoint {
            x: ((viewport_bottom_right.x + margin) / tile_size_f64).ceil()
                as i64,
            y: ((viewport_bottom_right.y + margin) / tile_size_f64).ceil()
                as i64,
        };

        let size = (bottom_right - &top_left).to_size().expect("bad size");

        IRect {
            top_left: top_left,
            size: size,
        }
    }

    /// Determine if the terrain tiles need updating, and return the new terrain
    /// rect, if an update is needed
    fn terrain_updates_required(&self, viewport_rect: &Rect) -> Option<IRect> {
        let mut terrain_rect =
            self.viewport_rect_to_terrain_rect(viewport_rect);

        if self.tile_terrain_coverage.contains_rect(&terrain_rect) {
            return None;
        }

        // If the natural size of the terrain-tile rect for the given viewport
        // rect is larger or equal to the size of the terrain-tiles array in
        // both dimensions, then the natual result is returned, but if the
        // natural rect is smaller in one or both dimensions, the natural
        // viewport will be centered within a viewport with the current size
        // (in tiles)
        if terrain_rect.size.width <= self.terrain_tiles_size.width {
            let dw = self.terrain_tiles_size.width - terrain_rect.size.width;
            terrain_rect.top_left.x -= dw as i64 / 2;
            terrain_rect.size.width = self.terrain_tiles_size.width;
        }

        if terrain_rect.size.height <= self.terrain_tiles_size.height {
            let dh = self.terrain_tiles_size.height - terrain_rect.size.height;
            terrain_rect.top_left.y -= dh as i64 / 2;
            terrain_rect.size.height = self.terrain_tiles_size.height;
        }

        Some(terrain_rect)
    }

    /// Add the two points together whithin the bounds of the tile grid system
    fn tile_grid_add(&self, lhs: &UPoint, rhs: &IPoint) -> UPoint {
        let tile_grid_width = self.terrain_tiles_size.width as i64;
        let tile_grid_height = self.terrain_tiles_size.height as i64;

        UPoint::new(
            (lhs.x as i64 + rhs.x).rem_euclid(tile_grid_width) as usize,
            (lhs.y as i64 + rhs.y).rem_euclid(tile_grid_height) as usize,
        )
    }

    /// Assume that the given point p is a point within the bounds of the
    /// tile grid, get the offset from origin of the tile grid (the top left)
    fn tile_grid_offset_from_origin(&self, p: &UPoint) -> IPoint {
        let grid_width = self.terrain_tiles_size.width as i64;
        let grid_height = self.terrain_tiles_size.height as i64;

        IPoint::new(
            (p.x as i64 - self.top_left_tile.x as i64).rem_euclid(grid_width),
            (p.y as i64 - self.top_left_tile.y as i64).rem_euclid(grid_height),
        )
    }

    // Subtract one grid point from another within the bounds of the grid
    fn tile_grid_sub(&self, lhs: &UPoint, rhs: &UPoint) -> IPoint {
        &self.tile_grid_offset_from_origin(lhs)
            - &self.tile_grid_offset_from_origin(rhs)
    }

    /// Determine which portion if any of the existing valid terrain tiles will
    /// remain valid when the viewport shifts to the given terrain rect, and
    /// return the new top-left point within the tile array, and the region of
    /// the tile array that's still valid
    fn calculate_new_valid_tiles(
        &self,
        new_terrain_rect: &IRect,
    ) -> Option<(UPoint, URect, IRect)> {
        self.tile_terrain_coverage
            .intersection(new_terrain_rect)
            .map(|itx_rect| {
                // Determine the top left and size of the valid region
                let valid_top_left_shift =
                    &itx_rect.top_left - &self.tile_terrain_coverage.top_left;

                let valid_tiles_top_left = self
                    .tile_grid_add(&self.top_left_tile, &valid_top_left_shift);

                let new_valid_rect = URect {
                    top_left: valid_tiles_top_left,
                    size: itx_rect.size.clone(),
                };

                // determine the new top left of the viewport rect

                let new_top_left_shift = &new_terrain_rect.top_left
                    - &self.tile_terrain_coverage.top_left;

                let new_top_left = self
                    .tile_grid_add(&self.top_left_tile, &new_top_left_shift);

                debug!("valid tile area = {}", new_valid_rect.area());

                (new_top_left, new_valid_rect, itx_rect)
            })
    }

    /// Get the tile at the given point in the tile grid using the given natural
    /// origin.  Also return the terrain coordinates of the tile based on the
    /// configured top-left tile in the grid and the tile-terrain coverage
    fn get_tile_at<'a>(
        &'a self,
        natural_origin: &UPoint,
        natural_x: &usize,
        natural_y: &usize,
    ) -> (&'a T, IPoint) {
        let real_point = UPoint::new(
            (natural_origin.x + natural_x) % self.terrain_tiles_size.width,
            (natural_origin.y + natural_y) % self.terrain_tiles_size.height,
        );

        let tile = self.get_tile(&real_point).unwrap_or_else(|| {
            error!("Invalid tile_coordinate {:?}", real_point);
            panic!("Index out of bounds error in terrain tiles array");
        });

        let terrain_point = &self.tile_terrain_coverage.top_left
            + &self.tile_grid_offset_from_origin(&real_point);

        (tile, terrain_point)
    }

    /// Update the invalid terrain tiles
    fn update_terrain_tiles(
        &self,
        new_valid_rect: URect,
        tile_updater: impl Fn(&T, &IPoint),
    ) {
        debug!("Updating terrain tiles");

        // hit all the partial rows to the right of the valid region

        let new_valid_size = &new_valid_rect.size;
        let valid_top_left = &new_valid_rect.top_left;

        let action = |x: usize, y: usize| {
            let (tile, terrain_point) =
                self.get_tile_at(valid_top_left, &x, &y);
            tile.set_location_point(&(&terrain_point * self.tile_size as f64));
            tile_updater(tile, &terrain_point);
        };

        for y in 0..new_valid_size.height {
            for x in new_valid_size.width..self.terrain_tiles_size.width {
                action(x, y);
            }
        }

        // hit all the complete rows below the valid region

        for y in new_valid_size.height..self.terrain_tiles_size.height {
            for x in 0..self.terrain_tiles_size.width {
                action(x, y);
            }
        }
    }

    /// return true if the current size of the 2d vector array is bigger than
    /// or equal to the size given in both height and width
    fn check_size_increased(&self, min_size: &ISize) -> bool {
        min_size.width > self.terrain_tiles_size.width
            || min_size.height > self.terrain_tiles_size.height
    }

    /// Update the information about the terrain tiles that describes which part
    /// of the terrain is being shown and where in the tile array the top-left
    /// corner is
    fn update_terrain_tiles_location(
        &mut self,
        new_terrain_rect: IRect,
        top_left_tile: UPoint,
    ) {
        self.top_left_tile = top_left_tile;
        self.tile_terrain_coverage = new_terrain_rect;
    }

    /// Increase the size of the 2d array of terrain tiles to accomodate the
    /// given size
    fn increase_size_for(
        &mut self,
        new_terrain_rect: IRect,
        tile_source: impl Fn() -> T,
    ) -> (URect, IRect) {
        let min_size = &new_terrain_rect.size;

        if !self.check_size_increased(min_size) {
            // ^ double checked lock
            debug!("Double checked lock failed");
            return URect {
                top_left: self.top_left_tile.clone(),
                size: self.terrain_tiles_size.clone(),
            };
        }

        debug!(
            "Increasing terrain tiles cache to {:?}",
            &new_terrain_rect.size
        );

        let width_inc = min_size
            .width
            .checked_sub(self.terrain_tiles_size.width)
            .unwrap_or_default();
        let height_inc = min_size
            .height
            .checked_sub(self.terrain_tiles_size.height)
            .unwrap_or_default();

        let tile_size_f64 = self.tile_size as f64;

        let tile_source_with_vis_and_size = || {
            let result = tile_source();
            result.set_size(tile_size_f64, tile_size_f64);
            result.set_visible(true);
            result
        };

        if width_inc > 0 {
            self.increase_row_width_by(
                width_inc,
                &tile_source_with_vis_and_size,
            );
        }

        if height_inc > 0 {
            self.increase_row_count_by(
                height_inc,
                &tile_source_with_vis_and_size,
            );
        }

        let (new_top_left, valid_rect, valid_terrain_rect) = self
            .calculate_new_valid_tiles(&new_terrain_rect)
            .unwrap_or_default();

        self.update_terrain_tiles_location(new_terrain_rect, new_top_left);

        (valid_rect, valid_terrain_rect)
    }

    /// Icnrease the size of all the existing rows in the terrain to the given
    /// width
    fn increase_row_width_by(
        &mut self,
        cols_to_add: usize,
        tile_source: &impl Fn() -> T,
    ) {
        let insert_point = &self.top_left_tile.x;

        self.terrain_tiles.iter_mut().for_each(|row| {
            let to_insert = iter::repeat_with(tile_source).take(cols_to_add);

            row.splice(insert_point..insert_point, to_insert)
                .for_each(|_| {});
        });

        self.top_left_tile.x += cols_to_add;
        self.terrain_tiles_size.width += cols_to_add;
    }

    /// Increase the number of rows in the sprite array by the given number
    fn increase_row_count_by(
        &mut self,
        rows_to_add: usize,
        tile_source: &impl Fn() -> T,
    ) {
        let total_columns = self.terrain_tiles_size.width;

        let new_rows = iter::repeat_with(|| {
            iter::repeat_with(tile_source).take(total_columns).collect()
        })
        .take(rows_to_add);

        let insert_point = &self.top_left_tile.y;

        self.terrain_tiles
            .splice(insert_point..insert_point, new_rows);

        self.top_left_tile.y += rows_to_add;
        self.terrain_tiles_size.height += rows_to_add;
    }

    fn get_tile<'a>(&'a self, point: &UPoint) -> Option<&'a T> {
        self.terrain_tiles
            .get(point.y)
            .and_then(|row| row.get(point.x))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
        Inner::new(DEFAULT_TILE_SIZE, 0.)
    }

    #[test]
    fn test_terrain_updates_required() {
        let mut this = default_test_terrain_generator();

        let mut viewport_rect = Rect::default();
        let mut new_terrain_rect =
            this.terrain_updates_required(&viewport_rect);

        assert_eq!(new_terrain_rect, None);

        let tile_size_f64 = this.tile_size as f64;

        viewport_rect.size.width = 2. * tile_size_f64;
        viewport_rect.size.height = 3. * tile_size_f64;
        viewport_rect.top_left.x = 0.5 * tile_size_f64;
        viewport_rect.top_left.y = -1.5 * tile_size_f64;

        new_terrain_rect = this.terrain_updates_required(&viewport_rect);

        assert_eq!(new_terrain_rect, Some(IRect::new(0, -2, 3, 4)));

        viewport_rect.top_left.x = 0.1 * tile_size_f64;
        viewport_rect.top_left.y = 0.9 * tile_size_f64;
        viewport_rect.size.width = 1.91 * tile_size_f64;
        viewport_rect.size.height = 3. * tile_size_f64;

        new_terrain_rect = this.terrain_updates_required(&viewport_rect);

        assert_eq!(new_terrain_rect, Some(IRect::new(0, 0, 3, 4)));

        // verify the centering of smaller viewports in larger tile buffers
        this = default_test_terrain_generator();

        viewport_rect.top_left.x = -2. * tile_size_f64;
        viewport_rect.top_left.y = -2. * tile_size_f64;
        viewport_rect.size.width = 4. * tile_size_f64;
        viewport_rect.size.height = 4. * tile_size_f64;

        new_terrain_rect = this.terrain_updates_required(&viewport_rect);

        assert_eq!(new_terrain_rect, Some(IRect::new(-2, -2, 4, 4)));

        let terrain_rect = new_terrain_rect.as_ref().map(Clone::clone).unwrap();

        this.increase_size_for(terrain_rect, Default::default);

        viewport_rect.top_left.x = -2. * tile_size_f64;
        viewport_rect.top_left.y = -2. * tile_size_f64;
        viewport_rect.size.width = 2. * tile_size_f64;
        viewport_rect.size.height = 2. * tile_size_f64;

        new_terrain_rect = this.terrain_updates_required(&viewport_rect);

        assert_eq!(new_terrain_rect, None);

        viewport_rect.top_left.x = 1. * tile_size_f64;
        viewport_rect.top_left.y = 1. * tile_size_f64;
        viewport_rect.size.width = 2. * tile_size_f64;
        viewport_rect.size.height = 2. * tile_size_f64;

        new_terrain_rect = this.terrain_updates_required(&viewport_rect);

        assert_eq!(new_terrain_rect, Some(IRect::new(0, 0, 4, 4)));
    }

    #[test]
    fn test_size_increased() {
        let mut this = default_test_terrain_generator();

        let mut viewport_rect = Rect::default();

        let tile_size_f64 = this.tile_size as f64;

        viewport_rect.top_left.x = -2. * tile_size_f64;
        viewport_rect.top_left.y = -2. * tile_size_f64;
        viewport_rect.size.width = 4. * tile_size_f64;
        viewport_rect.size.height = 4. * tile_size_f64;

        let new_terrain_rect = this.terrain_updates_required(&viewport_rect);

        assert_eq!(new_terrain_rect, Some(IRect::new(-2, -2, 4, 4)));

        let terrain_rect = new_terrain_rect.as_ref().cloned().unwrap();

        assert!(this.check_size_increased(&new_terrain_rect.unwrap().size));

        this.increase_size_for(terrain_rect, Default::default);

        assert_eq!(this.tile_terrain_coverage, IRect::new(-2, -2, 4, 4));
    }

    #[test]
    fn test_calculate_vew_valid_rect() {
        let mut this = default_test_terrain_generator();

        let mut viewport_rect = Rect::default();

        let tile_size_f64 = this.tile_size as f64;

        viewport_rect.top_left.x = -2. * tile_size_f64;
        viewport_rect.top_left.y = -2. * tile_size_f64;
        viewport_rect.size.width = 4. * tile_size_f64;
        viewport_rect.size.height = 4. * tile_size_f64;

        let new_terrain_rect = this.terrain_updates_required(&viewport_rect);

        assert_eq!(new_terrain_rect, Some(IRect::new(-2, -2, 4, 4)));

        let terrain_rect = new_terrain_rect.as_ref().cloned().unwrap();

        assert!(this.check_size_increased(&new_terrain_rect.unwrap().size));

        this.increase_size_for(terrain_rect, Default::default);

        assert_eq!(this.tile_terrain_coverage, IRect::new(-2, -2, 4, 4));

        assert_eq!(
            this.calculate_new_valid_tiles(&this.tile_terrain_coverage),
            Some((UPoint::default(), URect::new(0, 0, 4, 4)))
        );

        assert_eq!(
            this.calculate_new_valid_tiles(&IRect::new(2, 2, 4, 4)),
            None
        );

        assert_eq!(
            this.calculate_new_valid_tiles(&IRect::new(1, 0, 4, 4)),
            Some((UPoint::new(3, 2), URect::new(3, 2, 1, 2)))
        );

        let (new_top_left, new_valid_rect) = this
            .calculate_new_valid_tiles(&IRect::new(-3, -4, 4, 4))
            .unwrap_or_default();

        assert_eq!(UPoint::new(3, 2), new_top_left);
        assert_eq!(URect::new(0, 0, 3, 2), new_valid_rect);

        this.update_terrain_tiles_location(
            IRect::new(-3, -4, 4, 4),
            new_top_left,
        );

        this.update_terrain_tiles(new_valid_rect, |tile, _| {
            tile.updated.inc();
        });

        assert_eq!(
            this.get_tile(&UPoint::new(0, 0)).map(|t| t.updated.get()),
            Some(0)
        );
        assert_eq!(
            this.get_tile(&UPoint::new(0, 1)).map(|t| t.updated.get()),
            Some(0)
        );
        assert_eq!(
            this.get_tile(&UPoint::new(0, 2)).map(|t| t.updated.get()),
            Some(1)
        );
        assert_eq!(
            this.get_tile(&UPoint::new(0, 3)).map(|t| t.updated.get()),
            Some(1)
        );
        assert_eq!(
            this.get_tile(&UPoint::new(1, 0)).map(|t| t.updated.get()),
            Some(0)
        );
        assert_eq!(
            this.get_tile(&UPoint::new(1, 1)).map(|t| t.updated.get()),
            Some(0)
        );
        assert_eq!(
            this.get_tile(&UPoint::new(1, 2)).map(|t| t.updated.get()),
            Some(1)
        );
        assert_eq!(
            this.get_tile(&UPoint::new(1, 3)).map(|t| t.updated.get()),
            Some(1)
        );
        assert_eq!(
            this.get_tile(&UPoint::new(2, 0)).map(|t| t.updated.get()),
            Some(0)
        );
        assert_eq!(
            this.get_tile(&UPoint::new(2, 1)).map(|t| t.updated.get()),
            Some(0)
        );
        assert_eq!(
            this.get_tile(&UPoint::new(2, 2)).map(|t| t.updated.get()),
            Some(1)
        );
        assert_eq!(
            this.get_tile(&UPoint::new(2, 3)).map(|t| t.updated.get()),
            Some(1)
        );
        assert_eq!(
            this.get_tile(&UPoint::new(3, 0)).map(|t| t.updated.get()),
            Some(1)
        );
        assert_eq!(
            this.get_tile(&UPoint::new(3, 1)).map(|t| t.updated.get()),
            Some(1)
        );
        assert_eq!(
            this.get_tile(&UPoint::new(3, 2)).map(|t| t.updated.get()),
            Some(1)
        );
        assert_eq!(
            this.get_tile(&UPoint::new(3, 3)).map(|t| t.updated.get()),
            Some(1)
        );

        assert_eq!(
            this.get_tile(&UPoint::new(3, 3))
                .map(|t| t.location.borrow().clone()),
            Some(Point::new(-96., -96.))
        );
    }
}
