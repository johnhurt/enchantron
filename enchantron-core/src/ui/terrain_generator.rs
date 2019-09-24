use std::sync::{Arc, RwLock};

use crate::model::{IRect, ISize};
use crate::native::RuntimeResources;
use crate::view_types::ViewTypes;

use super::{SpriteSource, SpriteSourceWrapper};

const TILE_SIZE: f64 = 16.;

pub struct TerrainGenerator<T>
where
    T: ViewTypes,
{
    sprite_source: SpriteSourceWrapper<T>,
    runtime_resources: Arc<RuntimeResources<T::SystemView>>,
    inner: RwLock<Inner<T>>,
}

struct Inner<T>
where
    T: ViewTypes,
{
    vec_size: ISize,
    sprites: Vec<Vec<T::Sprite>>,
}

impl<T> TerrainGenerator<T>
where
    T: ViewTypes,
{
    pub fn new(
        sprite_source: SpriteSourceWrapper<T>,
        runtime_resources: Arc<RuntimeResources<T::SystemView>>,
    ) -> TerrainGenerator<T> {
        TerrainGenerator {
            sprite_source: sprite_source,
            runtime_resources: runtime_resources,

            inner: RwLock::new(Inner::new()),
        }
    }

    fn with_inner<R>(&self, action: impl FnOnce(&Inner<T>) -> R) -> R {
        let ref inner = self.inner.read().unwrap_or_else(|err| {
            error!("Failed to get read lock on inner terrain map: {:?}", err);
            panic!("Failed to get a read lock on the inner state");
        });

        action(&*inner)
    }

    fn with_inner_mut<R>(&self, action: impl FnOnce(&mut Inner<T>) -> R) -> R {
        let ref mut inner = self.inner.write().unwrap_or_else(|err| {
            error!("Failed to get write lock on inner terrain map: {:?}", err);
            panic!("Failed to get a write lock on the inner state");
        });

        action(&mut *inner)
    }

    pub fn on_viewport_change(&self, veiport_rect: &IRect) {
        self.sprite_source.create_sprite();
    }

    fn ensure_size(&self, min_size: &ISize) {
        if !self.with_inner(|inner| inner.check_size(min_size)) {
            self.with_inner_mut(|inner| {
                inner.increase_size_for(min_size, &self.sprite_source)
            })
        }
    }
}

impl<T> Inner<T>
where
    T: ViewTypes,
{
    pub fn new() -> Inner<T> {
        Inner {
            sprites: Default::default(),
            vec_size: Default::default(),
        }
    }

    /// return true if the current size of the 2d vector array is bigger than
    /// or equal to the size given in both height and width
    fn check_size(&self, min_size: &ISize) -> bool {
        min_size.width <= self.vec_size.width
            && min_size.height <= self.vec_size.height
    }

    /// Increase the size of the 2d array of sprites to accomodate the given
    /// size
    fn increase_size_for(
        &mut self,
        min_size: &ISize,
        sprite_source: &SpriteSourceWrapper<T>,
    ) {
        if !self.check_size(min_size) {
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
        self.sprites.iter_mut().for_each(|row| {
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

            self.sprites.push(new_row);
        }
    }
}
