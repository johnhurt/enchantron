use crate::native::{SystemView, Texture, TextureLoader};
use crate::ui::{
    Button, ProgressBar, Sprite, SpriteGroup, SpriteSource, Viewport,
};
use crate::view::{GameView, LoadingView, MainMenuView};

use std::ops::Deref;

pub trait DynSpriteSource:
    'static
    + Deref<Target = dyn SpriteSource<T = Self::Tx, S = Self::Sx, G = Self::Gx>>
    + Clone
    + Send
    + Sync
    + Unpin
{
    type Tx: Texture;
    type Sx: Sprite;
    type Gx: SpriteGroup;
}

pub trait ViewTypes: 'static + Send + Sync {
    type Texture: Texture;
    type TextureLoader: TextureLoader<T = Self::Texture>;
    type Sprite: Sprite<T = Self::Texture>;
    type SpriteGroup: SpriteGroup<
        S = Self::Sprite,
        T = Self::Texture,
        G = Self::SpriteGroup,
    >;
    type Viewport: Viewport<
        S = Self::Sprite,
        T = Self::Texture,
        G = Self::SpriteGroup,
    >;
    type GameView: GameView<
        S = Self::Sprite,
        V = Self::Viewport,
        T = Self::Texture,
        G = Self::SpriteGroup,
    >;
    type Button: Button;
    type ProgressBar: ProgressBar;
    type LoadingView: LoadingView<P = Self::ProgressBar>;
    type MainMenuView: MainMenuView<B = Self::Button>;
    type SystemView: SystemView<T = Self::Texture, TL = Self::TextureLoader>;

    // Kind of a kluge way of creating a dynamic trait object
    type DynSpriteSource: DynSpriteSource<
        Tx = Self::Texture,
        Sx = Self::Sprite,
        Gx = Self::SpriteGroup,
    >;
}
