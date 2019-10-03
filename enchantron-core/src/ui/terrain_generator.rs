use std::sync::{Arc, Mutex, RwLock};
use std::iter;

use crate::event::{
    EnchantronEvent, EventBus, EventListener, HasListenerRegistrations,
    ListenerRegistration, ViewportChange,
};
use crate::model::{IPoint, UPoint, ISize, Rect, Size, Point, IRect, URect};
use crate::native::{RuntimeResources, Textures};
use crate::view_types::ViewTypes;

use super::{SpriteSource, SpriteSourceWrapper, HasMutableSize};

pub const DEFAULT_TILE_SIZE: usize = 32;

pub struct TerrainGenerator<T>
where
    T: ViewTypes,
{
    sprite_source: SpriteSourceWrapper<T>,
    runtime_resources: Arc<RuntimeResources<T::SystemView>>,
    listener_registrations: Mutex<Vec<ListenerRegistration>>,
    inner: RwLock<Inner<T>>,
}

struct Inner<T>
where
    T: ViewTypes,
{
    terrain_tiles_size: ISize,
    terrain_tiles: Vec<Vec<T::Sprite>>,
    tile_size: usize,
    tile_terrain_coverage: IRect,
    top_left_tile: UPoint
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

            inner: RwLock::new(Inner::new(DEFAULT_TILE_SIZE)),
        });

        event_bus.register(ViewportChange::default(), Arc::downgrade(&result));

        result
    }

    /// Run the given action with a read-only reference to the inner terrain
    /// generator
    fn with_inner<R>(&self, action: impl FnOnce(&Inner<T>) -> R) -> R {
        let ref inner = self.inner.read().unwrap_or_else(|err| {
            error!("Failed to get read lock on inner terrain map: {:?}", err);
            panic!("Failed to get a read lock on the inner state");
        });

        action(&*inner)
    }

    /// Run the given action with a rw reference to the inner terrain
    /// generator
    fn with_inner_mut<R>(&self, action: impl FnOnce(&mut Inner<T>) -> R) -> R {
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

        let terrain_rect = self.with_inner(|inner| {
            inner.viewport_rect_to_terrain_rect(viewport_rect)
        });

        if !self.with_inner(|inr| inr.terrain_updates_required(&terrain_rect)) {
            return;
        }

        let valid_tile_size = {
            let min_size = &terrain_rect.size;

            if !self.with_inner(|inner| inner.check_size_increased(min_size)) {
                self.with_inner_mut(|inner| {
                    inner.increase_size_for(terrain_rect, &self.sprite_source)
                })
            }
            else {
                let new_valid_rect = self.with_inner(|inner| {
                    inner.calculate_new_valid_tiles(&terrain_rect)
                })
                .unwrap_or_default();

                let URect { size: valid_size, top_left } = new_valid_rect;

                self.with_inner_mut(|inner| {
                    inner.update_terrain_tiles_location(terrain_rect, top_left);
                });

                valid_size
            }
        };

        self.with_inner(|inner| {
            inner.update_terrain_tiles(
                valid_tile_size,
                self.runtime_resources.textures());
        });
    }
}

impl<T> Inner<T>
where
    T: ViewTypes,
{
    pub fn new(tile_size: usize) -> Inner<T> {
        Inner {
            terrain_tiles: Default::default(),
            terrain_tiles_size: Default::default(),
            tile_size: tile_size,
            tile_terrain_coverage: Default::default(),
            top_left_tile: Default::default()
        }
    }

    /// Get the tile size required to support the given viewport size based
    /// on the configured size of the tiles
    fn viewport_size_to_tile_size(&self, viewport_size: &Size) -> ISize {
        let tile_size_f64 = self.tile_size as f64;
        ISize::new(
            ((viewport_size.width / tile_size_f64).floor() + 1.) as usize,
            ((viewport_size.height / tile_size_f64).floor() + 1.) as usize,
        )
    }

    /// Get the terrain rect required to cover the given viewport rect based on
    /// the current size of the terrain tiles array.  If the natural size of the
    /// terrain-tile rect for the given viewport rect is larger or equal to the
    /// size of the terrain-tiles array in both dimensions, then the natual
    /// result is returned, but if the natural rect is smaller in one or both
    /// dimensions, the natural viewport will be centered within a viewport with
    /// the current size (in tiles)
    fn viewport_rect_to_terrain_rect(&self, viewport_rect: &Rect) -> IRect {
        let mut result = IRect {
            size: self.viewport_size_to_tile_size(
                &viewport_rect.size),
            top_left: self.terrain_point_for_viewport_point(
                &viewport_rect.top_left)
        };

        if result.size.width <= self.terrain_tiles_size.width {
            let dw = self.terrain_tiles_size.width - result.size.width;
            result.top_left.x = dw as i64 / 2;
            result.size.width = self.terrain_tiles_size.width;
        }

        if result.size.height <= self.terrain_tiles_size.height {
            let dh = self.terrain_tiles_size.height - result.size.height;
            result.top_left.y -= dh as i64 / 2;
            result.size.height = self.terrain_tiles_size.height;
        }

        result
    }

    /// Get the coordinates of the terrain tile containing the given viewport
    /// point.
    fn terrain_point_for_viewport_point(&self, viewport_point: &Point) -> IPoint {
        let tile_size_f64 = self.tile_size as f64;
        IPoint {
            x: (viewport_point.x / tile_size_f64).floor() as i64,
            y: (viewport_point.y / tile_size_f64).floor() as i64,
        }
    }

    /// Determine if the terrain tiles need updating
    fn terrain_updates_required(&self, new_terrain_rect: &IRect) -> bool {
        !self.tile_terrain_coverage.contains_rect(new_terrain_rect)
    }

    /// Get the location of the top-left corner of the tile that contains the
    /// given point
    fn get_top_left_of_tile_containing(&self, point: &Point) -> Point {
        let tile_size_f64 = self.tile_size as f64;
        Point::new(
            (point.x / tile_size_f64).floor() * tile_size_f64,
            (point.y / tile_size_f64).floor() * tile_size_f64,
        )
    }

    /// Determine which portion if any of the existing valid terrain tiles will
    /// remain valid when the viewport shifts to the given terrain rect
    fn calculate_new_valid_tiles(&self, new_terrain_rect: &IRect)
            -> Option<URect> {

        self.tile_terrain_coverage.intersection(new_terrain_rect).map(|itx_rect| {

            let top_left_shift = &itx_rect.top_left
                - &self.tile_terrain_coverage.top_left;

            let mut new_valid_rect = URect::default();

            {
                let tile_grid_size = &self.terrain_tiles_size;
                let valid_region_top_left = &mut new_valid_rect.top_left;

                valid_region_top_left.x
                    = (valid_region_top_left.x + top_left_shift.x as usize)
                        % tile_grid_size.width;

                valid_region_top_left.y
                    = (valid_region_top_left.y + top_left_shift.y as usize)
                        % tile_grid_size.height;
            }

            let valid_tiles_size = &mut new_valid_rect.size;

            valid_tiles_size.width -= top_left_shift.x as usize;
            valid_tiles_size.height -= top_left_shift.y as usize;

            new_valid_rect
        })
    }

    /// Update the tile at the given natural coordinates
    fn update_tile(&self,
            natural_x: &usize,
            natural_y: &usize,
            terrain_point: &IPoint,
            textures: &Textures<T::Texture>) {
        let real_x = ( self.top_left_tile.x + natural_x )
                % self.terrain_tiles_size.width;
        let real_y = ( self.top_left_tile.y + natural_y )
                % self.terrain_tiles_size.height;

        debug!("Updating Tile {}, {}, {:?}", real_x, real_y, terrain_point);
    }

    /// Update the invalid terrain tiles
    fn update_terrain_tiles(&self, new_valid_size: ISize, textures: &Textures<T::Texture>) {
        // hit all the partial rows to the right of the valid region

        let top_left = &self.tile_terrain_coverage.top_left;
        let mut terrain_point : IPoint = Default::default();

        for y in 0..new_valid_size.height {
            for x in new_valid_size.width..self.terrain_tiles_size.width {
                terrain_point.x = top_left.x + x as i64;
                terrain_point.y = top_left.y + y as i64;
                self.update_tile(&x, &y, &terrain_point);
            }
        }

        // hit all the complete rows below the valid region

        for y in new_valid_size.height..self.terrain_tiles_size.height {
            for x in 0..self.terrain_tiles_size.width {
                terrain_point.x = top_left.x + x as i64;
                terrain_point.y = top_left.y + y as i64;
                self.update_tile(&x, &y, &terrain_point);
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
    fn update_terrain_tiles_location(&mut self,
        new_terrain_rect: IRect,
        top_left_tile: UPoint ) {

        self.top_left_tile = top_left_tile;
        self.tile_terrain_coverage = new_terrain_rect;
    }

    /// Increase the size of the 2d array of terrain tiles to accomodate the
    /// given size
    fn increase_size_for(
        &mut self,
        new_terrain_rect: IRect,
        sprite_source: &SpriteSourceWrapper<T>,
    ) -> ISize {
        let min_size = &new_terrain_rect.size;

        if !self.check_size_increased(min_size) {
            // ^ double checked lock
            return self.terrain_tiles_size.clone();
        }

        let width_inc = min_size.width
            .checked_sub(self.terrain_tiles_size.width)
            .unwrap_or_default();
        let height_inc = min_size.height
            .checked_sub(self.terrain_tiles_size.height)
            .unwrap_or_default();

        let tile_size_f64 = self.tile_size as f64;

        let sprite_source_with_size = || {
            let result = sprite_source.create_sprite();
            result.set_size(tile_size_f64, tile_size_f64);
            result
        };

        if width_inc > 0 {
            self.increase_row_width_by(width_inc, &sprite_source_with_size);
        }

        if height_inc > 0 {
            self.increase_row_count_by(height_inc, &sprite_source_with_size);
        }

        self.calculate_new_valid_tiles(&new_terrain_rect)
            .map(|valid_tiles| {
                let URect { size, top_left: new_top_left } = valid_tiles;
                self.update_terrain_tiles_location(
                        new_terrain_rect,
                        new_top_left);
                size
            })
            .unwrap_or_default()
    }

    /// Icnrease the size of all the existing rows in the terrain to the given
    /// width
    fn increase_row_width_by(
        &mut self,
        cols_to_add: usize,
        sprite_source: &impl Fn() -> T::Sprite
    ) {

        let insert_point = &self.top_left_tile.x;

        self.terrain_tiles.iter_mut().for_each(|row| {
            let to_insert = iter::repeat_with(sprite_source)
                .take(cols_to_add);

            row.splice(insert_point..insert_point, to_insert);
        });

        self.top_left_tile.x += cols_to_add;
        self.terrain_tiles_size.width += cols_to_add;
    }

    /// Increase the number of rows in the sprite array by the given number
    fn increase_row_count_by(
        &mut self,
        rows_to_add: usize,
        sprite_source: &impl Fn() -> T::Sprite
    ) {

        let total_columns = self.terrain_tiles_size.width;

        let new_rows = iter::repeat_with(|| {
            iter::repeat_with(sprite_source).take(total_columns).collect()
        })
        .take(rows_to_add);

        let insert_point = &self.top_left_tile.y;

        self.terrain_tiles.splice(insert_point..insert_point, new_rows);

        self.top_left_tile.y += rows_to_add;
        self.terrain_tiles_size.height += rows_to_add;
    }
}
