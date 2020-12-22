use crate::native::{Animation, ResourceLoader, SystemView, Texture};
use crate::ui::{Color, Sprite, SpriteGroup, Viewport};
use crate::view::NativeView;

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
    type SystemView: SystemView<T = Self::Texture, TL = Self::ResourceLoader>;
}
