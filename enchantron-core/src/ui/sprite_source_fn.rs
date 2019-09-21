pub type SpriteSourceFn<S> = Box<dyn Fn() -> S + 'static + Send + Sync>;
