#[macro_export]
macro_rules! view_impl {

    ($impl_name:ident<$view_types_generic:ident> : $view_name:ident {
        $(
            let $field_name:ident : $field_type:ty;
        )*
    }) => {

        pub type $impl_name<$view_types_generic> = hidden::ViewImpl<$view_types_generic>;

        mod hidden {
            use crate::ui::*;
            use crate::view_types::ViewTypes;
            use super::*;
            use crate::view::NativeView;

            #[derive(derive_new::new)]
            pub struct ViewImpl<$view_types_generic> where $view_types_generic : ViewTypes {
                view_impl: $view_types_generic::NativeView,
                $(
                    $field_name: $field_type
                ),*
            }

            impl <T> NativeView for $impl_name<T> where T : ViewTypes {
                fn unset_presenter(&self) {
                    self.view_impl.unset_presenter()
                }

                fn set_presenter(&self, presenter: crate::util::BoxedAny) {
                    self.view_impl.set_presenter(presenter)
                }
            }

            impl <V> SpriteSource for $impl_name<V> where V : ViewTypes {
                type T = <V::NativeView as SpriteSource>::T;
                type S = <V::NativeView as SpriteSource>::S;
                type G = <V::NativeView as SpriteSource>::G;

                fn create_sprite(&self) -> Self::S {
                    self.view_impl.create_sprite()
                }

                fn create_group(&self) -> Self::G {
                    self.view_impl.create_group()
                }
            }

            impl <T> HasViewport for $impl_name<T> where T : ViewTypes {
                type V =  <T::NativeView as HasViewport>::V;

                fn get_viewport(&self) -> Self::V {
                    self.view_impl.get_viewport()
                }
            }

            impl <T> HasMagnifyHandlers for $impl_name<T> where T : ViewTypes {
                type R = <T::NativeView as HasMagnifyHandlers>::R ;

                fn add_magnify_handler(&self, magnify_handler: MagnifyHandler) -> Self::R {
                    self.view_impl.add_magnify_handler(magnify_handler)
                }
            }

            impl <T> HasMultiTouchHandlers for $impl_name<T> where T : ViewTypes {
                type R = <T::NativeView as HasMultiTouchHandlers>::R ;

                fn add_multi_touch_handler(&self, multi_touch_handler: MultiTouchHandler) -> Self::R {
                    self.view_impl.add_multi_touch_handler(multi_touch_handler)
                }
            }

            impl <T> HasLayoutHandlers for $impl_name<T> where T : ViewTypes {
                type R = <T::NativeView as HasLayoutHandlers>::R ;

                fn add_layout_handler(&self, layout_handler: LayoutHandler) -> Self::R {
                    self.view_impl.add_layout_handler(layout_handler)
                }
            }
        }

    }

}
