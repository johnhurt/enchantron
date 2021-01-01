use crate::native::Texture;
use crate::ui::{Sprite, SpriteGroup, SpriteSource};
use std::any::Any;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;

pub type WidgetSelector<W> =
    Box<dyn Fn(&mut dyn Any) -> &mut W + Send + Sync + 'static>;

pub type AnyConsumer = Box<dyn FnOnce(&mut dyn Any) + Send + Sync + 'static>;

#[derive(derive_new::new)]
pub struct WidgetPublicMessageSink<W> {
    pub(crate) sender: Sender<AnyConsumer>,
    pub(crate) selector: WidgetSelector<W>,
}

pub trait Widget: Sized {
    type T: Texture;
    type S: Sprite<T = Self::T>;
    type G: SpriteGroup<S = Self::S, T = Self::T>;

    fn new(
        sprite_source: &impl SpriteSource<S = Self::S, T = Self::T, G = Self::G>,
        sink: WidgetPublicMessageSink<Self>,
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
            use crate::ui::{ Widget, WidgetPublicMessageSink, SpriteSource };
            use super::*;
            use crate::view_types::ViewTypes;
            use std::sync::Arc;

            #[derive(derive_new::new)]
            pub struct WidgetPublic<T: ViewTypes> {
                sink: Arc<WidgetPublicMessageSink<WidgetPrivate<T>>>
            }

            pub struct WidgetPrivate<$view_types_generic: ViewTypes> {
                public: WidgetPublic<$view_types_generic>,
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
                    let copy = self.clone();
                    let _ = self
                        .sink
                        .sender
                        .try_send(Box::new(move |any| action((copy.sink.selector)(any))));
                }
            }

            impl <T> Widget for WidgetPrivate<T> where T : ViewTypes {
                type T = T::Texture;
                type S = T::Sprite;
                type G = T::SpriteGroup;

                fn new(
                    sprite_source: &impl SpriteSource<S = Self::S, T = Self::T, G = Self::G>,
                    sink: WidgetPublicMessageSink<WidgetPrivate<T>>
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
                        public,
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
                pub fn public(&self) -> WidgetPublic<T> {
                    self.public.clone()
                }
            }
        }

    };
}
