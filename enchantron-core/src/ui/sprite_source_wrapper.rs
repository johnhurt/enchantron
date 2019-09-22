use std::sync::Arc;

use super::SpriteSource;
use crate::view_types::ViewTypes;

type SpriteSourceFn<S> = dyn Fn() -> S + 'static + Send + Sync;

pub struct SpriteSourceWrapper<T: ViewTypes>(Arc<SpriteSourceFn<T::Sprite>>);

impl<T> SpriteSourceWrapper<T>
where
    T: ViewTypes,
{
    pub fn new(
        source: impl Fn() -> T::Sprite + 'static + Send + Sync,
    ) -> SpriteSourceWrapper<T> {
        SpriteSourceWrapper(Arc::new(source))
    }
}

impl<T> SpriteSource for SpriteSourceWrapper<T>
where
    T: ViewTypes,
{
    type T = T::Texture;
    type S = T::Sprite;

    fn create_sprite(&self) -> T::Sprite {
        (self.0)()
    }
}

impl<T> Clone for SpriteSourceWrapper<T>
where
    T: ViewTypes,
{
    fn clone(&self) -> SpriteSourceWrapper<T> {
        SpriteSourceWrapper(self.0.clone())
    }
}
