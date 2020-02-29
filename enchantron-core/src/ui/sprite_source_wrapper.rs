use std::sync::Arc;

use super::SpriteSource;
use crate::view_types::ViewTypes;

type SpriteSourceFn<S> = dyn Fn() -> S + 'static + Send + Sync;

pub struct SpriteSourceWrapper<T: ViewTypes>(
    Arc<SpriteSourceFn<T::Sprite>>,
    Arc<SpriteSourceFn<T::SpriteGroup>>,
);

impl<T> SpriteSourceWrapper<T>
where
    T: ViewTypes,
{
    pub fn new(real_source: &impl SpriteSource) -> SpriteSourceWrapper<T> {
        SpriteSourceWrapper(
            Arc::new(|| real_source.create_sprite()),
            Arc::new(|| real_source.create_group()),
        )
    }
}

impl<T> SpriteSource for SpriteSourceWrapper<T>
where
    T: ViewTypes,
{
    type T = T::Texture;
    type S = T::Sprite;
    type G = T::SpriteGroup;

    fn create_sprite(&self) -> T::Sprite {
        (self.0)()
    }

    fn create_group(&self) -> T::SpriteGroup {
        (self.1)()
    }
}

impl<T> Clone for SpriteSourceWrapper<T>
where
    T: ViewTypes,
{
    fn clone(&self) -> SpriteSourceWrapper<T> {
        SpriteSourceWrapper(self.0.clone(), self.1.clone())
    }
}
