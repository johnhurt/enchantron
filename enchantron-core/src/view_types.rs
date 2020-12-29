use crate::native::{Animation, ResourceLoader, SystemInterop, Texture};
use crate::ui::{
    Color, ProgressBar, Sprite, SpriteGroup, TransitionService, Viewport,
};
use crate::view::{LoadingView, NativeView};

pub trait ViewTypes: 'static + Send + Sync + Unpin {
    type Color: Color;
    type Texture: Texture;
    type Animation: Animation<Texture = Self::Texture>;
    type ResourceLoader: ResourceLoader<T = Self::Texture, A = Self::Animation>;
    type Sprite: Sprite<T = Self::Texture, A = Self::Animation, C = Self::Color>;
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
    type NativeView: NativeView<
        S = Self::Sprite,
        V = Self::Viewport,
        T = Self::Texture,
        G = Self::SpriteGroup,
    >;
    type LoadingView: LoadingView<
        S = Self::Sprite,
        V = Self::Viewport,
        T = Self::Texture,
        G = Self::SpriteGroup,
        P = Self::ProgressBar,
    >;
    type ProgressBar: ProgressBar;
    type TransitionService: TransitionService<
        NV = Self::NativeView,
        LV = Self::LoadingView,
    >;
    type SystemInterop: SystemInterop<
        T = Self::Texture,
        TL = Self::ResourceLoader,
        TS = Self::TransitionService,
        NV = Self::NativeView,
        LV = Self::LoadingView,
    >;
}
