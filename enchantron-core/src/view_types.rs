use crate::native::{Animation, ResourceLoader, Shader, SystemView, Texture};
use crate::ui::{Button, ProgressBar, Sprite, SpriteGroup, Viewport};
use crate::view::{GameView, LoadingView, MainMenuView};

pub trait ViewTypes: 'static + Send + Sync + Unpin {
    type Texture: Texture;
    type Animation: Animation<Texture = Self::Texture>;
    type Shader: Shader;
    type ResourceLoader: ResourceLoader<
        T = Self::Texture,
        A = Self::Animation,
        S = Self::Shader,
    >;
    type Sprite: Sprite<
        T = Self::Texture,
        A = Self::Animation,
        S = Self::Shader,
    >;
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
    type SystemView: SystemView<T = Self::Texture, TL = Self::ResourceLoader>;
}
