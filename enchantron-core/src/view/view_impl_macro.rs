#[macro_export]
macro_rules! widget_private_type {
    ($widget_type:ident<$view_types_generic:ident>) => {
        paste::paste! {
            [< $widget_type Private >]<$view_types_generic>
        }
    };
}

#[macro_export]
macro_rules! widget_public_type {
    ($widget_type:ident<$view_types_generic:ident>) => {
        paste::paste! {
            [< $widget_type Public >]<$view_types_generic>
        }
    };
}

#[macro_export]
macro_rules! widget_constructor {
    ($widget_type:ident<$view_types_generic:ident>(
        $container:expr,
        $sender:ident,
        $widget_field:ident)) => {
        paste::paste! {
            [< $widget_type Private >]::new(
                $container,
                $sender.clone(),
                Arc::new(|view_as_any: &mut dyn Any| {
                    &mut view_as_any
                        .downcast_mut::<ViewPrivate<$view_types_generic>>()
                        .unwrap()
                        .$widget_field
                }),
            )
        }
    };
}

#[macro_export]
macro_rules! view_impl {
    ($view_name:ident<$view_types_generic:ident> {
        $(
            widgets {$(
                $widget_field:ident : $widget_type:ident
            ),* $(,)?}
        )?
        $(
            private {$(
                $private_field:ident : $private_field_type:ty
            ),* $(,)?}
        )?
        $(init = $init_fn:ident;)?
        $(on_layout = $on_layout_fn:ident;)?
    }
    ) => {
        paste::paste! {
            pub type [< $view_name Public >]<$view_types_generic> = hidden::ViewPublic<$view_types_generic>;
            pub type [< $view_name Private >]<$view_types_generic> = hidden::ViewPrivate<$view_types_generic>;
        }

        mod hidden {
            use crate::ui::*;
            use crate::model::*;
            use crate::view_types::ViewTypes;
            use super::*;
            use crate::view::{NativeView, AnyConsumer};
            use std::sync::Arc;
            use tokio::sync::mpsc::{channel, Sender};
            use crate::{widget_public_type, widget_private_type, widget_constructor};

            pub struct ViewPrivate<$view_types_generic: ViewTypes> {
                $($(
                    pub(crate) $widget_field: widget_private_type!($widget_type<$view_types_generic>),
                )*)?
                $($(
                    pub(crate) $private_field : $private_field_type
                ),*)?
            }

            pub struct ViewPublic<T: ViewTypes> {
                pub(crate) inner: Arc<Inner<T>>
            }

            pub struct Inner<$view_types_generic> where $view_types_generic : ViewTypes {
                registrations: Vec<Box<dyn HandlerRegistration>>,
                pub(crate) raw_view: $view_types_generic::NativeView
                $($(
                    , pub(crate) $widget_field: widget_public_type!($widget_type<$view_types_generic>)
                )*)?
            }

            impl <$view_types_generic> ViewPublic<$view_types_generic>
                where $view_types_generic : ViewTypes
            {

                pub fn new(
                    raw_view: $view_types_generic::NativeView
                    $($(
                        , $private_field : $private_field_type
                    )*)?
                ) -> ViewPublic<T> {

                    let (raw_sender, mut receiver) = channel::<AnyConsumer>(32);
                    let sender = Arc::new(raw_sender);

                    $($(

                        let $widget_field = widget_constructor!(
                            $widget_type<$view_types_generic>(
                                &raw_view,
                                sender,
                                $widget_field
                            )
                        );
                    )*)?

                    let mut registrations = Vec::<Box<dyn HandlerRegistration>>::new();

                    $(
                        let layout_sender = sender.clone();
                        let layout_handler = create_layout_handler!(|width, height| {
                            let _ = layout_sender.try_send(Box::new(
                                move |view_as_any| {
                                    view_as_any
                                        .downcast_mut::<ViewPrivate<$view_types_generic>>()
                                        .unwrap()
                                        .$on_layout_fn(Size::new(width as f64, height as f64))
                                }
                            ));
                        });

                        registrations.push(Box::new(raw_view.add_layout_handler(layout_handler)));
                    )?

                    let public = ViewPublic {
                        inner: Arc::new(Inner {
                            registrations,
                            raw_view
                            $($(
                                , $widget_field: $widget_field.public()
                            )*)?
                        })
                    };

                    let private = ViewPrivate {
                        $($(
                            $widget_field,
                        )*)?
                        $($(
                            $private_field,
                        )*)?
                    };

                    let _ = tokio::spawn(async move {

                        let mut private = private;

                        $(private.$init_fn();)?

                        while let Some(action) = receiver.recv().await {
                            action(&mut private);
                        }

                    });

                    public
                }

            }

            impl <T> NativeView for ViewPublic<T> where T : ViewTypes {
                fn unset_presenter(&self) {
                    self.inner.raw_view.unset_presenter()
                }

                fn set_presenter(&self, presenter: crate::util::BoxedAny) {
                    self.inner.raw_view.set_presenter(presenter)
                }
            }

            impl <V> SpriteSource for ViewPublic<V> where V : ViewTypes {
                type T = <V::NativeView as SpriteSource>::T;
                type S = <V::NativeView as SpriteSource>::S;
                type G = <V::NativeView as SpriteSource>::G;

                fn create_sprite(&self) -> Self::S {
                    self.inner.raw_view.create_sprite()
                }

                fn create_group(&self) -> Self::G {
                    self.inner.raw_view.create_group()
                }
            }

            impl <T> HasViewport for ViewPublic<T> where T : ViewTypes {
                type V =  <T::NativeView as HasViewport>::V;

                fn get_viewport(&self) -> Self::V {
                    self.inner.raw_view.get_viewport()
                }
            }

            impl <T> HasMagnifyHandlers for ViewPublic<T> where T : ViewTypes {
                type R = <T::NativeView as HasMagnifyHandlers>::R ;

                fn add_magnify_handler(&self, magnify_handler: MagnifyHandler) -> Self::R {
                    self.inner.raw_view.add_magnify_handler(magnify_handler)
                }
            }

            impl <T> HasMultiTouchHandlers for ViewPublic<T> where T : ViewTypes {
                type R = <T::NativeView as HasMultiTouchHandlers>::R ;

                fn add_multi_touch_handler(&self, multi_touch_handler: MultiTouchHandler) -> Self::R {
                    self.inner.raw_view.add_multi_touch_handler(multi_touch_handler)
                }
            }

            impl <T> HasLayoutHandlers for ViewPublic<T> where T : ViewTypes {
                type R = <T::NativeView as HasLayoutHandlers>::R ;

                fn add_layout_handler(&self, layout_handler: LayoutHandler) -> Self::R {
                    self.inner.raw_view.add_layout_handler(layout_handler)
                }
            }
        }
    };

}
