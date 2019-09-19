use super::Sprite;

pub type SpriteSourceFn<S: Sprite> = Box<dyn Fn() -> S + 'static + Send + Sync>;
