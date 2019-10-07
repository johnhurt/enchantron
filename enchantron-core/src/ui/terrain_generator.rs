use std::iter;
use std::sync::{Arc, Mutex, RwLock};

use crate::event::{
    EnchantronEvent, EventBus, EventListener, HasListenerRegistrations,
    ListenerRegistration, ViewportChange,
};
use crate::model::{IPoint, IRect, ISize, Point, Rect, Size, UPoint, URect};
use crate::native::{RuntimeResources, Textures};
use crate::view_types::ViewTypes;

use super::{
    HasMutableLocation, HasMutableSize, HasMutableVisibility, Sprite,
    SpriteSource, SpriteSourceWrapper,
};

pub const DEFAULT_TILE_SIZE: usize = 32;

pub struct TerrainGenerator<T>
where
    T: ViewTypes,
{
    sprite_source: SpriteSourceWrapper<T>,
    runtime_resources: Arc<RuntimeResources<T::SystemView>>,
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
    tile_terrain_coverage: IRect,
    top_left_tile: UPoint,
}

impl<T> HasListenerRegistrations for TerrainGenerator<T>
where
    T: ViewTypes,
{
    fn add_listener_registration(
        &self,
        listener_registration: ListenerRegistration,
    ) {
        if let Ok(mut locked_list) = self.listener_registrations.lock() {
            info!("Adding listener registration");
            locked_list.push(listener_registration);
        } else {
            error!("Failed to add listener registration");
        }
    }
}

impl<T> EventListener<EnchantronEvent, ViewportChange> for TerrainGenerator<T>
where
    T: ViewTypes,
{
    fn on_event(&self, event: &ViewportChange) {
        info!("Viewport changed : {:?}", event.new_viewport_rect);

        self.on_viewport_change(&event.new_viewport_rect);
    }
}

impl<T> TerrainGenerator<T>
where
    T: ViewTypes,
{
    pub fn new(
        event_bus: EventBus<EnchantronEvent>,
        sprite_source: SpriteSourceWrapper<T>,
        runtime_resources: Arc<RuntimeResources<T::SystemView>>,
    ) -> Arc<TerrainGenerator<T>> {
        let result = Arc::new(TerrainGenerator {
            sprite_source: sprite_source,
            runtime_resources: runtime_resources,

            listener_registrations: Default::default(),

            inner: RwLock::new(Inner::default()),
        });

        event_bus.register(ViewportChange::default(), Arc::downgrade(&result));

        result
    }

    /// Run the given action with a read-only reference to the inner terrain
    /// generator
    fn with_inner<R>(&self, action: impl FnOnce(&Inner<T::Sprite>) -> R) -> R {
        let ref inner = self.inner.read().unwrap_or_else(|err| {
            error!("Failed to get read lock on inner terrain map: {:?}", err);
            panic!("Failed to get a read lock on the inner state");
        });

        action(&*inner)
    }

    /// Run the given action with a rw reference to the inner terrain
    /// generator
    fn with_inner_mut<R>(
        &self,
        action: impl FnOnce(&mut Inner<T::Sprite>) -> R,
    ) -> R {
        let ref mut inner = self.inner.write().unwrap_or_else(|err| {
            error!("Failed to get write lock on inner terrain map: {:?}", err);
            panic!("Failed to get a write lock on the inner state");
        });

        action(&mut *inner)
    }

    /// Called when the viewport changes to adjust the terrain.  This method
    /// 1. Checks to see if the terrain needs to be updated to contain the
    ///    given viewport, and if not, the method returns
    /// 2. checks to see if the size of the terrain tiles is big enough to
    ///    contain the viewport rect given
    /// 3. if the terrain tiles needs to be altered, increase the size of the
    ///    terrain tiles array and updates
    /// 4. ?
    fn on_viewport_change(&self, viewport_rect: &Rect) {
        let terrain_rect_opt = self
            .with_inner(|inner| inner.terrain_updates_required(viewport_rect));

        if terrain_rect_opt.is_none() {
            debug!("No terrain updates");
            return;
        }

        let terrain_rect = terrain_rect_opt.unwrap();

        let valid_tile_size = {
            let min_size = &terrain_rect.size;

            if self.with_inner(|inner| inner.check_size_increased(min_size)) {
                debug!("Size increased");
                self.with_inner_mut(|inner| {
                    inner.increase_size_for(terrain_rect, || {
                        self.sprite_source.create_sprite()
                    })
                })
            } else {
                debug!("Size not increased");
                let new_valid_rect = self
                    .with_inner(|inner| {
                        inner.calculate_new_valid_tiles(&terrain_rect)
                    })
                    .unwrap_or_default();

                let URect {
                    size: valid_size,
                    top_left,
                } = new_valid_rect;

                self.with_inner_mut(|inner| {
                    inner.update_terrain_tiles_location(terrain_rect, top_left);
                });

                valid_size
            }
        };

        let grass = self.runtime_resources.textures().overworld.grass();
        let dirt = self.runtime_resources.textures().overworld.dirt();

        self.with_inner(|inner| {
            inner.update_terrain_tiles(valid_tile_size, |tile, point| {
                if point.x % 2 == 0 && point.y % 2 == 0 {
                    tile.set_texture(grass)
                }
                else {
                    tile.set_texture(dirt)
                }
            });
        });
    }
}

impl<T> Default for Inner<T>
where
    T: Sized + HasMutableLocation + HasMutableSize + HasMutableVisibility,
{
    fn default() -> Inner<T> {
        Inner::new(DEFAULT_TILE_SIZE)
    }
}

impl<T> Inner<T>
where
    T: Sized + HasMutableLocation + HasMutableSize + HasMutableVisibility,
{
    pub fn new(tile_size: usize) -> Inner<T> {
        Inner {
            terrain_tiles: Default::default(),
            terrain_tiles_size: Default::default(),
            tile_size: tile_size,
            tile_terrain_coverage: Default::default(),
            top_left_tile: Default::default(),
        }
    }

    /// Get the terrain rect required to cover the given viewport rect based on
    /// the current size of the terrain tiles array.
    fn viewport_rect_to_terrain_rect(&self, viewport_rect: &Rect) -> IRect {
        let tile_size_f64 = self.tile_size as f64;

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

    /// Determine which portion if any of the existing valid terrain tiles will
    /// remain valid when the viewport shifts to the given terrain rect
    fn calculate_new_valid_tiles(
        &self,
        new_terrain_rect: &IRect,
    ) -> Option<URect> {
        self.tile_terrain_coverage
            .intersection(new_terrain_rect)
            .map(|itx_rect| {
                let top_left_shift =
                    &itx_rect.top_left - &self.tile_terrain_coverage.top_left;

                let mut new_valid_rect = URect::default();

                {
                    let tile_grid_size = &self.terrain_tiles_size;
                    let valid_region_top_left = &mut new_valid_rect.top_left;

                    valid_region_top_left.x = (valid_region_top_left.x
                        + top_left_shift.x as usize)
                        % tile_grid_size.width;

                    valid_region_top_left.y = (valid_region_top_left.y
                        + top_left_shift.y as usize)
                        % tile_grid_size.height;
                }

                let valid_tiles_size = &mut new_valid_rect.size;

                valid_tiles_size.width -= top_left_shift.x as usize;
                valid_tiles_size.height -= top_left_shift.y as usize;

                new_valid_rect
            })
    }

    /// Update the tile at the given natural coordinates
    fn get_tile_at<'a>(
        &'a self,
        natural_x: &usize,
        natural_y: &usize,
    ) -> &'a T {
        let real_col =
            (self.top_left_tile.x + natural_x) % self.terrain_tiles_size.width;
        let real_row =
            (self.top_left_tile.y + natural_y) % self.terrain_tiles_size.height;

        self.terrain_tiles
            .get(real_row)
            .and_then(|row| row.get(real_col))
            .unwrap_or_else(|| {
                error!("Invalid row, col ({}, {}) ", real_col, real_row);
                panic!("Index out of bounds error in terrain tiles array");
            })
    }

    /// Update the invalid terrain tiles
    fn update_terrain_tiles(
        &self,
        new_valid_size: ISize,
        tile_updater: impl Fn(&T, &IPoint),
    ) {
        debug!("Updating terrain tiles");

        // hit all the partial rows to the right of the valid region

        let top_left = &self.tile_terrain_coverage.top_left;
        let mut terrain_point: IPoint = Default::default();

        for y in 0..new_valid_size.height {
            for x in new_valid_size.width..self.terrain_tiles_size.width {
                terrain_point.x = top_left.x + x as i64;
                terrain_point.y = top_left.y + y as i64;
                let tile = self.get_tile_at(&x, &y);
                tile_updater(tile, &terrain_point);
                tile.set_location_point(
                    &(&terrain_point * self.tile_size as f64),
                );
            }
        }

        // hit all the complete rows below the valid region

        for y in new_valid_size.height..self.terrain_tiles_size.height {
            for x in 0..self.terrain_tiles_size.width {
                terrain_point.x = top_left.x + x as i64;
                terrain_point.y = top_left.y + y as i64;
                let tile = self.get_tile_at(&x, &y);
                tile_updater(tile, &terrain_point);
                tile.set_location_point(
                    &(&terrain_point * self.tile_size as f64),
                );
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
    ) -> ISize {
        let min_size = &new_terrain_rect.size;

        if !self.check_size_increased(min_size) {
            // ^ double checked lock
            debug!("Double checked lock failed");
            return self.terrain_tiles_size.clone();
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

        let (new_top_left, valid_size) = self
            .calculate_new_valid_tiles(&new_terrain_rect)
            .map(|valid_tiles| {
                let URect {
                    size: valid_size,
                    top_left: new_top_left,
                } = valid_tiles;
                (new_top_left, valid_size)
            })
            .unwrap_or_default();

        self.update_terrain_tiles_location(
            new_terrain_rect,
            new_top_left,
        );

        valid_size
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

            row.splice(insert_point..insert_point, to_insert).for_each(|_|{});
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    #[derive(Default)]
    struct TestTile {
        size: RefCell<Size>,
        location: RefCell<Point>,
        visible: RefCell<bool>,
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
        Inner::default()
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

        let mut new_terrain_rect = this.terrain_updates_required(&viewport_rect);

        assert_eq!(new_terrain_rect, Some(IRect::new(-2, -2, 4, 4)));

        let terrain_rect = new_terrain_rect.as_ref().cloned().unwrap();

        assert!(this.check_size_increased(&new_terrain_rect.unwrap().size));

        this.increase_size_for(terrain_rect, Default::default);

        assert_eq!(this.tile_terrain_coverage, IRect::new(-2, -2, 4, 4));
    }
}
