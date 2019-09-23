use std::sync::{Arc, RwLock};

use crate::model::{ISize, IRectpoo};
use crate::native::RuntimeResources;
use crate::view_types::ViewTypes;

use super::{SpriteSource, SpriteSourceWrapper};

pub struct TerrainGenerator<T>
where
    T: ViewTypes,
{
    sprite_source: SpriteSourceWrapper<T>,
    runtime_resources: Arc<RuntimeResources<T::SystemView>>,
    inner: RwLock<Inner<T>>
}

#[derive(Default)]
struct Inner<T> {
    vec_size: ISize,
    sprites: Vec<Vec<T::Sprite>>
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

        }
    }

    pub fn on_viewport_change(&self, veiport_rect: &Rect) {
        self.sprite_source.create_sprite();
    }

    fn ensure_size(&self, min_size: &ISize) {
        if ! self.inner.check_size() {
            self.inner.increase_size_for(min_size, &self.sprite_source)
        }
    }
}

impl <T> Inner<T> where T: ViewTypes {

    /// return true if the current size of the 2d vector array is bigger than
    /// or equal to the size given in both height and width
    fn check_size(&self, min_size: &ISize) -> bool {
        min_size.width <= self.vec_size.width
            && min_size.height <= self.vec_size.height
    }

    fn increase_size_for(&mut self, min_size: &ISize, sprite_source: &SpriteSourceWrapper<T>) {
        if !self.check_size(min_size) { // <- double checked lock
            if min_size.width > self.vec_size.width {
                let cols_to_add = min_size.width - self.vec_size.width;
                self.increase_row_width(cols_to_add, sprite_source);
                self.vec_size.width = min_size.width;
            }
        }
    }

    /// Icnrease the size of all the existing rows in the terrain to the given
    /// width
    fn increase_row_width_by(&mut self, cols_to_add: usize, sprite_source: &SpriteSourceWrapper<T>) {
        self.sprites.iter_mut().for_each(|row| {
            for _ in 0..cols_to_add {
                row.push(sprite_source.create_sprite());
            }
        })
    }

}
