use crate::native::{SystemView, Texture, TextureLoader};
use crate::ui::{Sprite, Viewport};
use crate::view::GameView;

pub trait ViewTypes: 'static + Send + Sync {
    type Texture: Texture;
    type TextureLoader: TextureLoader<T = Self::Texture>;
    type Sprite: Sprite<T = Self::Texture>;
    type Viewport: Viewport<S = Self::Sprite, T = Self::Texture>;
    type GameView: GameView<
        S = Self::Sprite,
        V = Self::Viewport,
        T = Self::Texture,
    >;
    type SystemView: SystemView<T = Self::Texture, TL = Self::TextureLoader>;
}
