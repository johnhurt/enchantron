use crate::native::Texture;
use crate::ui::{Sprite, SpriteGroup, SpriteSource};
use crate::util::DynActionSink;

pub trait Widget: Sized {
    type T: Texture;
    type S: Sprite<T = Self::T>;
    type G: SpriteGroup<S = Self::S, T = Self::T>;

    fn new(
        sprite_source: &impl SpriteSource<S = Self::S, T = Self::T, G = Self::G>,
        sink: DynActionSink<Self>,
    ) -> Self;
}

#[macro_export]
macro_rules! widget {
    ($widget_trait:ident<$view_types_generic:ident> {
        $(
            sprites {$(
                $sprite_name:ident
            ),*}
        )?

        $(
            private {$(
                $field_name:ident : $field_type:ty
            ),*}
        )?

    }) => {

        paste::paste!{
            pub type [<$widget_trait Public>]<$view_types_generic>
                = hidden::WidgetPublic<$view_types_generic>;
            pub type [<$widget_trait Private>]<$view_types_generic>
                = hidden::WidgetPrivate<$view_types_generic>;
        }

        mod hidden {
            use crate::ui::{ Widget, SpriteSource };
            use super::*;
            use crate::view_types::ViewTypes;
            use std::sync::Arc;
            use crate::util::DynActionSink;

            #[derive(derive_new::new)]
            pub struct WidgetPublic<T: ViewTypes> {
                pub(crate) sink: Arc<DynActionSink<WidgetPrivate<T>>>
            }

            pub struct WidgetPrivate<$view_types_generic: ViewTypes> {
                public: Option<WidgetPublic<$view_types_generic>>,
                $($(
                    pub(crate) $sprite_name: $view_types_generic::Sprite,
                )*)?
                $($(
                    pub(crate) $field_name : $field_type
                ),*)?
            }

            impl <T> Clone for WidgetPublic<T> where T: ViewTypes {
                fn clone(&self) -> Self {
                    WidgetPublic {
                        sink: self.sink.clone()
                    }
                }
            }

            impl <T> WidgetPublic<T> where T: ViewTypes {
                pub fn send(
                    &self,
                    action: impl FnOnce(&mut WidgetPrivate<T>) + Send + Sync + 'static,
                ) {
                    self.sink.send(action);
                }
            }

            impl <T> Widget for WidgetPrivate<T> where T : ViewTypes {
                type T = T::Texture;
                type S = T::Sprite;
                type G = T::SpriteGroup;

                fn new(
                    sprite_source: &impl SpriteSource<S = Self::S, T = Self::T, G = Self::G>,
                    sink: DynActionSink<WidgetPrivate<T>>
                ) -> WidgetPrivate<T>
                {
                    $($(
                        let $sprite_name = sprite_source.create_sprite();
                    )*)?
                    $($(
                        let $field_name = Default::default();
                    )*)?

                    let public = WidgetPublic::new(Arc::new(sink));

                    WidgetPrivate {
                        public: Some(public),
                        $($(
                            $sprite_name,
                        )*)?
                        $($(
                            $field_name
                        ),*)?
                    }
                }
            }

            impl <T> WidgetPrivate<T> where T: ViewTypes {
                pub fn public(&mut self) -> WidgetPublic<T> {
                    self.public.take().expect("Cannot take public more than once")
                }
            }
        }

    };
}
