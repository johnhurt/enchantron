#[macro_export]
macro_rules! view_impl {

    ($impl_name:ident : $view_name:ident {
        $(
            let $field_name:ident : $field_type:ty;
        )*

        $(
            fn $pass_through_func:ident($(
                $func_arg_name:ident: $func_arg_type:ty
            ),*) $( -> $func_return:ty )?;
        )*
    }) => {

        pub type $impl_name<T> = hidden::ViewImpl<T>;

        mod hidden {
            use crate::ui::*;
            use crate::view_types::ViewTypes;
            use super::*;
            use crate::view::{$view_name, BaseView};

            pub struct ViewImpl<T> where T : ViewTypes {
                view_impl: T::$view_name,
                $(
                    $field_name: $field_type
                ),*
            }

            impl <T> $view_name for $impl_name<T> where T : ViewTypes {
                $(
                    fn $pass_through_func(&self, $(
                        $func_arg_name: $func_arg_type
                    ),*) $( -> $func_return )? {
                        self.view_impl.$pass_through_func($($func_arg_name)*)
                    }
                )*
            }

            impl <T> BaseView for $impl_name<T> where T : ViewTypes {
                fn initialize_pre_bind(&self) {
                    self.view_impl.initialize_pre_bind()
                }

                fn initialize_post_bind(&self, presenter: crate::util::BoxedAny) {
                    self.view_impl.initialize_post_bind(presenter)
                }
            }

            impl <V> SpriteSource for $impl_name<V> where V : ViewTypes {
                type T = <V::$view_name as SpriteSource>::T;
                type S = <V::$view_name as SpriteSource>::S;
                type G = <V::$view_name as SpriteSource>::G;

                fn create_sprite(&self) -> Self::S {
                    self.view_impl.create_sprite()
                }

                fn create_group(&self) -> Self::G {
                    self.view_impl.create_group()
                }
            }

            impl <T> HasViewport for $impl_name<T> where T : ViewTypes {
                type V =  <T::$view_name as HasViewport>::V;

                fn get_viewport(&self) -> Self::V {
                    self.view_impl.get_viewport()
                }
            }

            impl <T> HasMagnifyHandlers for $impl_name<T> where T : ViewTypes {
                type R = <T::$view_name as HasMagnifyHandlers>::R ;

                fn add_magnify_handler(&self, magnify_handler: MagnifyHandler) -> Self::R {
                    self.view_impl.add_magnify_handler(magnify_handler)
                }
            }

            impl <T> HasMultiTouchHandlers for $impl_name<T> where T : ViewTypes {
                type R = <T::$view_name as HasMultiTouchHandlers>::R ;

                fn add_multi_touch_handler(&self, multi_touch_handler: MultiTouchHandler) -> Self::R {
                    self.view_impl.add_multi_touch_handler(multi_touch_handler)
                }
            }

            impl <T> HasLayoutHandlers for $impl_name<T> where T : ViewTypes {
                type R = <T::$view_name as HasLayoutHandlers>::R ;

                fn add_layout_handler(&self, layout_handler: LayoutHandler) -> Self::R {
                    self.view_impl.add_layout_handler(layout_handler)
                }
            }
        }

    }

}
