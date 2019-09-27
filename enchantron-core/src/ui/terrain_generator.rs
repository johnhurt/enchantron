use std::sync::{Arc, Mutex, RwLock};

use crate::event::{
    EnchantronEvent, EventBus, EventListener, HasListenerRegistrations,
    ListenerRegistration, ViewportChange,
};
use crate::model::{IPoint, IRect, ISize, Rect, Size};
use crate::native::RuntimeResources;
use crate::view_types::ViewTypes;

use super::{SpriteSource, SpriteSourceWrapper};

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
    vec_size: ISize,
    terrain_tiles: Vec<Vec<T::Sprite>>,
    tile_size: usize,
    top_left_tile: IPoint,
    tile_terrain_coverage: Rect,
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
    /// 3. if the terrain tiles needs to be altered, the inner is locked and
    ///    resized.  Resizing the terrain tiles invalidates the textures on the
    ///    tiles because it can introduce unpredictable gaps, so the whole
    ///    terrain needs to be retextured.
    /// 4. ?
    fn on_viewport_change(&self, viewport_rect: &Rect) {
        if !self.with_inner(|inr| inr.terrain_updates_required(viewport_rect)) {
            return;
        }

        let tiles_size = self.with_inner(|inner| {
            inner.viewport_size_to_tile_size(&viewport_rect.size)
        });

        if !self.with_inner(|inner| inner.check_size_increased(&tiles_size)) {
            self.with_inner_mut(|inner| {
                inner.increase_size_for(&tiles_size, &self.sprite_source)
            });

            self.with_inner(|inner| {
                inner.fully_update_terrain_tiles(viewport_rect)
            });
        }
    }
}

impl<T> Inner<T>
where
    T: ViewTypes,
{
    pub fn new(tile_size: usize) -> Inner<T> {
        Inner {
            terrain_tiles: Default::default(),
            vec_size: Default::default(),
            tile_size: tile_size,
            top_left_tile: Default::default(),
            tile_terrain_coverage: Default::default(),
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

    /// Determine if the terrain tiles need updating
    fn terrain_updates_required(&self, new_viewport_rect: &Rect) -> bool {
        !self.tile_terrain_coverage.contains_rect(new_viewport_rect)
    }

    /// Get the location of the top-left corner of the tile that contains the
    /// given point
    fn get_top_left_of_tile_containing(&self, point: &Point) -> Point {
        let tile_size_f64 = self.tile_size as f64;
        Point::new(
            (point.x / tile_size_f64).floor() * tile_size_f64,
            (point.y / tile_size_f64).floor() * tile_size_f64,
        );
    }

    /// Fully update the terrain tiles based on the given size
    fn fully_update_terrain_tiles(&self, new_viewport_rect: &Rect) {}

    /// return true if the current size of the 2d vector array is bigger than
    /// or equal to the size given in both height and width
    fn check_size_increased(&self, min_size: &ISize) -> bool {
        min_size.width > self.vec_size.width
            || min_size.height > self.vec_size.height
    }

    /// Increase the size of the 2d array of terrain tiles to accomodate the
    /// given size
    fn increase_size_for(
        &mut self,
        min_size: &ISize,
        sprite_source: &SpriteSourceWrapper<T>,
    ) {
        if !self.check_size_increased(min_size) {
            // ^ double checked lock
            return;
        }

        if min_size.width > self.vec_size.width {
            let cols_to_add = min_size.width - self.vec_size.width;
            self.increase_row_width_by(cols_to_add, sprite_source);
            self.vec_size.width = min_size.width;
        }

        if min_size.height > self.vec_size.height {
            let rows_to_add = min_size.height - self.vec_size.height;
            self.increase_row_count_by(rows_to_add, sprite_source);
            self.vec_size.height = min_size.height;
        }

        self.top_left_tile = Default::default();
    }

    /// Icnrease the size of all the existing rows in the terrain to the given
    /// width
    fn increase_row_width_by(
        &mut self,
        cols_to_add: usize,
        sprite_source: &SpriteSourceWrapper<T>,
    ) {
        self.terrain_tiles.iter_mut().for_each(|row| {
            for _ in 0..cols_to_add {
                row.push(sprite_source.create_sprite());
            }
        })
    }

    /// Increase the number of rows in the sprite array by the given number
    fn increase_row_count_by(
        &mut self,
        rows_to_add: usize,
        sprite_source: &SpriteSourceWrapper<T>,
    ) {
        for _ in 0..rows_to_add {
            let mut new_row = vec![];
            for _ in 0..self.vec_size.height {
                new_row.push(sprite_source.create_sprite());
            }

            self.terrain_tiles.push(new_row);
        }
    }
}
