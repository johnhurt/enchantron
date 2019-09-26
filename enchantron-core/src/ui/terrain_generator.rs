use std::sync::{Arc, RwLock, Mutex };

use crate::event::{ EventBus, EnchantronEvent, ViewportChange, EventListener, ListenerRegistration, HasListenerRegistrations};
use crate::model::{IRect, ISize, IPoint, Rect, Size};
use crate::native::RuntimeResources;
use crate::view_types::ViewTypes;

use super::{SpriteSource, SpriteSourceWrapper};

pub const DEFAULT_TILE_SIZE : f64 = 32.;

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
    tile_size: f64,
    top_left_tile: IPoint,
    top_left_offset_in_tile: Point,
    currect_viewport_rect: Rect,
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

    // Called when the viewport changes to adjust the terrain
    fn on_viewport_change(&self, veiwport_rect: &Rect) {
        if self.with_inner(|inner| inner.terrain_updates_required(veiwport_rect)) {
            self.with_inner_mut(|inner| {
                inner.increase_size_for(min_size, &self.sprite_source)
            })
        }
    }

    // ensure that the size of the terrain tiles 2d array is gte the size given
    fn ensure_size(&self, min_size: &ISize) {

    }
}

impl<T> Inner<T>
where
    T: ViewTypes,
{
    pub fn new(tile_size: f64) -> Inner<T> {
        Inner {
            terrain_tiles: Default::default(),
            vec_size: Default::default(),
            tile_size: tile_size,
            top_left_tile: Default::default(),
            top_left_offset_in_tile: Default::default(),
            currect_viewport_rect: Default::default(),
        }
    }

    /// Get the tile size required to support the given viewport size based
    /// on the configured size of the tiles
    fn viewport_size_to_tile_size(&self, viewport_size: &Size) -> ISize {
        ISize::new(
            ((viewport_size.width / self.tile_size).floor() + 1.) as usize,
            ((viewport_size.height / self.tile_size).floor() + 1.) as usize)
    }

    /// determine if the terrain tiles need updating
    fn terrain_updates_required(&self, new_viewport_rect: &Rect) -> bool {
        let terrain_tile_size
                = self.viewport_size_to_tile_size(&new_viewport_rect.size);

        self.check_size_increased(&terrain_tile_size)
                || self.check_top_left_tile_changed(new_viewport_rect)
    }

    /// return true if the current size of the 2d vector array is bigger than
    /// or equal to the size given in both height and width
    fn check_size_increased(&self, min_size: &ISize) -> bool {
        min_size.width > self.vec_size.width
            || min_size.height > self.vec_size.height
    }

    fn check_top_left_tile_changed(&self, new_viewport_rect: &Rect) -> bool {
        let top_left_shift = &self.currect_viewport_rect.top_left
                - &new_viewport_rect.top_left;


    }

    /// Increase the size of the 2d array of terrain tiles to accomodate the
    /// given size
    fn increase_size_for(
        &mut self,
        min_size: &ISize,
        sprite_source: &SpriteSourceWrapper<T>,
    ) {
        if self.check_size_increased(min_size) {
            // ^ double checked lock

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
        }
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
