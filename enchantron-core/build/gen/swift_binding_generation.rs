use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::fs::{File, OpenOptions};
use std::path::Path;

use cbindgen;
use cbindgen::Language;
use handlebars::{handlebars_helper, Handlebars, StringWriter};
use heck::{MixedCase, SnakeCase};

use super::data_type::*;
use super::{
    ArgumentDefBuilder, FieldDefBuilder, GenericDefBuilder,
    ImplBlockDefBuilder, ImplDefBuilder, MethodDefBuilder, RenderableContext,
    RenderableType, TypeDef, TypeDefBuilder,
};

macro_rules! swift_struct {
    ($name:ident) => {
        DataType::swift_struct(stringify!($name), None)
    };
    (Self = $struct_name:ident) => {
        DataType::swift_generic(
            None,
            DataType::swift_struct(stringify!($struct_name), None),
        )
    };
    (Self::$generic_symbol:ident = $struct_name:ident) => {
        DataType::swift_generic(
            Some(stringify!($generic_symbol)),
            DataType::swift_struct(stringify!($struct_name), None),
        )
    };
}

macro_rules! arg {
    ($name:ident : $type_exp:expr) => {{
        let result = ArgumentDefBuilder::default()
            .name(stringify!($name))
            .data_type(Clone::clone(&$type_exp))
            .build()
            .unwrap();

        result
    }};
}

macro_rules! method_builder {
    ($name:ident($($arg_name:ident : $arg_type_exp:expr),* $(,)? ) $( -> $return_exp:expr)? ) => {
        {
            let result = MethodDefBuilder::default()
                .name(stringify!($name))
                .arguments(vec![$(
                    arg!($arg_name : $arg_type_exp)
                ),*]);

            $(
                let result = result.return_type(Some(Clone::clone(&$return_exp)));
            )?
            result
        }
    };
}

macro_rules! method {
    ($name:ident($($arg_name:ident : $arg_type_exp:expr),* $(,)? ) $( -> $return_exp:expr)? ) => {
        method_builder!($name($( $arg_name: $arg_type_exp),*) $( -> $return_exp)? ).build().unwrap()
    };
}

macro_rules! field {
    (mut $name:ident : $type_exp:expr) => {
        FieldDefBuilder::default()
            .name(stringify!($name))
            .data_type(Clone::clone(&$type_exp))
            .setter(true)
            .build()
            .unwrap()
    };

    ($name:ident : $type_exp:expr) => {
        FieldDefBuilder::default()
            .name(stringify!($name))
            .data_type(Clone::clone(&$type_exp))
            .build()
            .unwrap()
    };
}

macro_rules! impl_def {
    ($trait_name:ty {
        $(
            type $associated_type_name:ident = $real_type:ty;
        )*
    }) => {{
        let result =
            ImplDefBuilder::default().trait_name(stringify!($trait_name));

        let result = result.generics(vec![ $(
            GenericDefBuilder::default()
                .symbol(Some(stringify!($associated_type_name)))
                .bound_type(stringify!($real_type))
                .build().unwrap()
        ),*]);

        result.build().unwrap()
    }};
}

macro_rules! impl_method {
    ($trait_name:ty {
        fn $method_name:ident($($arg_name:ident : $arg_type_exp:expr),* $(,)? ) $( -> $return_exp:expr)?
    }) => {{
        let result = method_builder!($method_name($($arg_name: $arg_type_exp),*) $( -> $return_exp)?);
        let result = result.impl_block(Some(ImplBlockDefBuilder::default()
            .trait_name(stringify!($trait_name))
            .build().unwrap()));
        result.build().unwrap()
    }};
}

macro_rules! rust_type {
    ($name:ident $( : $rust_type:ty )? {
        $(exclude_from_header = $exclude_from_header:expr;)?
        $(
            fn $method_name:ident($($arg_name:ident : $arg_type_exp:expr),* $(,)? ) $( -> $return_exp:expr)?;
        )*
    }) => {{
        let result = TypeDefBuilder::default()
            .name(stringify!($name))
            .rust_owned(true)
            .methods(vec![$(
                method!($method_name($($arg_name:$arg_type_exp),*) $( -> $return_exp)?)
            ),*]);

        $(
            let result = result.exclude_from_header($exclude_from_header);
        )?
        $(
            let result = result.rust_import(Some(stringify!($rust_type)));
        )?

        result.build().unwrap()
    }}
}

macro_rules! swift_type {
    ($name:ident {
        $(
            custom_rust_drop_code = $custom_rust_drop_code:expr;
        )?
        $(
            let $($field_lhs_part:ident)+ : $field_type_exp:expr;
        )*
        $(
            fn $method_name:ident($($arg_name:ident : $arg_type_exp:expr),* $(,)? ) $( -> $return_exp:expr)?;
        )*
        $(
            impl $trait_name:ty {
                $(
                    type $associated_type_name:ident = $real_type:ty;
                )*
                $(
                    fn $impl_method_name:ident($($impl_arg_name:ident : $impl_arg_type_exp:expr),* $(,)? ) $( -> $impl_return_exp:expr)?;
                )*
            }
        )*
    }) => {{
        let result = TypeDefBuilder::default()
            .name(stringify!($name))
            .fields(vec![$(
                field!($($field_lhs_part )+ : $field_type_exp)
            )*])
            .methods(vec![$(
                method!($method_name($($arg_name:$arg_type_exp),*) $( -> $return_exp)?),
            )*
            $(
                $(
                    impl_method!($trait_name {
                        fn $impl_method_name($($impl_arg_name : $impl_arg_type_exp),*) $( -> $impl_return_exp)?
                     }),
                )*
            )*]);

        $(
            let result = result.custom_rust_drop_code(Some($custom_rust_drop_code));
        )?

        let result = result.impls(vec![$(
            impl_def!($trait_name { $( type $associated_type_name = $real_type; )* })
        ),*]);

        result.build().unwrap()
    }};
}

macro_rules! empty_type {
    ($name:ident  {
        $(
            impl $trait_name:ty {
                $(
                    type $associated_type_name:ident = $real_type:ty;
                )*
            }
        )*
    }) => {{
        let result = TypeDefBuilder::default()
            .name(stringify!($name));

        let result = result.impls(vec![$(
            impl_def!($trait_name { $( type $associated_type_name = $real_type; )* })
        ),*]);

        result.build().unwrap()
    }}
}

lazy_static! {

  #[derive(Serialize)]
  static ref TYPES : Vec<TypeDef> = vec![

    // Low-level Types
    rust_type!( ByteBuffer : crate::util::ByteBuffer {
        fn get_length() -> LONG;
        fn get_content() -> MUTABLE_BYTE_POINTER;
    } ),

    swift_type!( SwiftString {

        let length : LONG;

        fn get_content() -> MUTABLE_BYTE_POINTER;
    }),

    rust_type!(BoxedAny : crate::util::BoxedAny {}),

    // Application Root Object

    rust_type!( ApplicationContext {

        exclude_from_header = true;

        fn transition_to_loading_view(view: swift_struct!(LoadingView));
        fn transition_to_main_menu_view(view: swift_struct!(MainMenuView));
        fn transition_to_game_view(view: swift_struct!(GameView));
    }),

    // UI Components

    swift_type!( HandlerRegistration {
        custom_rust_drop_code = "crate::ui::HandlerRegistration::deregister(self);";

        impl crate::ui::HandlerRegistration {
            fn deregister();
        }
    }),

    rust_type!( ClickHandler : crate::ui::ClickHandler {
        fn on_click();
    }),

    rust_type!( MagnifyHandler : crate::ui::MagnifyHandler {
        fn on_magnify(scale_change_additive: DOUBLE, zoom_center_x: DOUBLE, zoom_center_y: DOUBLE);
    }),

    rust_type!(MultiTouchHandler : crate::ui::MultiTouchHandler {
        fn on_one_drag_start(drag_id: LONG, global_x: DOUBLE, global_y: DOUBLE, click_count: LONG);
        fn on_one_drag_move(drag_id: LONG, global_x: DOUBLE, global_y: DOUBLE, click_count: LONG);
        fn on_one_drag_end(drag_id: LONG, global_x: DOUBLE, global_y: DOUBLE, click_count: LONG);

        fn on_two_drags_start(drag_id_1: LONG, global_x_1: DOUBLE, global_y_1: DOUBLE, click_count_1: LONG,
                              drag_id_2: LONG, global_x_2: DOUBLE, global_y_2: DOUBLE, click_count_2: LONG);
        fn on_two_drags_move(drag_id_1: LONG, global_x_1: DOUBLE, global_y_1: DOUBLE, click_count_1: LONG,
                             drag_id_2: LONG, global_x_2: DOUBLE, global_y_2: DOUBLE, click_count_2: LONG);
        fn on_two_drags_end(drag_id_1: LONG, global_x_1: DOUBLE, global_y_1: DOUBLE, click_count_1: LONG,
                            drag_id_2: LONG, global_x_2: DOUBLE, global_y_2: DOUBLE, click_count_2: LONG);
    }),

    rust_type!(LayoutHandler : crate::ui::LayoutHandler {
        fn on_layout(width: LONG, height: LONG);
    }),

    empty_type!(ViewTypes {
        impl crate::view_types::ViewTypes {
            type Color = Color;
            type Sprite = Sprite;
            type SpriteGroup = SpriteGroup;
            type LoadingView = LoadingView;
            type Texture = Texture;
            type Animation = Animation;
            type ResourceLoader = ResourceLoader;
            type SystemView = SystemView;
            type GameView = GameView;
            type Viewport = Viewport;
            type MainMenuView = MainMenuView;
        }
    }),

    TypeDefBuilder::default()
        .name("TextArea")
        .rust_owned(false)
        .impls(vec![
            ImplDefBuilder::default()
                .trait_name("HasText")
                .trait_import(Some("crate::ui::HasText"))
                .build().unwrap()
        ])
        .fields(vec![
            FieldDefBuilder::default()
                .name("text")
                .getter_impl(Some(ImplBlockDefBuilder::default()
                    .trait_name("HasText")
                    .build().unwrap()))
                .setter_impl(Some(ImplBlockDefBuilder::default()
                    .trait_name("HasText")
                    .build().unwrap()))
                .data_type(*STRING)
                .setter(true)
                .build().unwrap()
        ])
        .build().unwrap(),

    swift_type!(Animation {
        impl crate::native::Animation {
            type Texture = Texture;

            fn add_texture(texture: swift_struct!( Self::Texture = Texture ));
            fn set_is_loop(is_loop: BOOLEAN);
            fn set_name(name: STRING);
        }
    }),

    swift_type!(Texture {
        impl crate::native::Texture {
            fn get_sub_texture(left: DOUBLE, top: DOUBLE, width: DOUBLE, weight: DOUBLE) -> swift_struct!(Self = Texture);
        }
        impl crate::ui::HasSize {
            fn get_width() -> DOUBLE;
            fn get_height() -> DOUBLE;
        }
    }),

    swift_type!(Sprite {
        impl crate::ui::Sprite {
            type T = Texture;
            type A = Animation;
            type C = Color;
        }

        impl crate::ui::HasMutableSize {

        }

        impl crate::ui::HasMutableColor {

        }

        impl crate::ui::HasMutableLocation {

        }

        impl crate::ui::HasMutableZLevel {

        }
    }),

    TypeDefBuilder::default()
        .name("Sprite")
        .rust_owned(false)
        .impls(vec![
            ImplDefBuilder::default()
                .trait_name("crate::ui::Sprite")
                .trait_import(Some("crate::ui"))
                .generics(vec![
                    GenericDefBuilder::default()
                        .symbol(Some("T"))
                        .bound_type("Texture")
                        .build().unwrap(),
                    GenericDefBuilder::default()
                        .symbol(Some("A"))
                        .bound_type("Animation")
                        .build().unwrap(),
                    GenericDefBuilder::default()
                        .symbol(Some("C"))
                        .bound_type("Color")
                        .build().unwrap()
                ])
                .build().unwrap(),

            ImplDefBuilder::default()
                .trait_name("HasMutableSize")
                .trait_import(Some("crate::ui::HasMutableSize"))
                .build().unwrap(),

            ImplDefBuilder::default()
                .trait_name("HasMutableColor")
                .trait_import(Some("crate::ui::HasMutableColor"))
                .generics(vec![
                    GenericDefBuilder::default()
                        .symbol(Some("C"))
                        .bound_type("Color")
                        .build().unwrap()
                ])
                .build().unwrap(),

            ImplDefBuilder::default()
                .trait_name("HasMutableLocation")
                .trait_import(Some("crate::ui::HasMutableLocation"))
                .build().unwrap(),

            ImplDefBuilder::default()
                .trait_name("HasMutableVisibility")
                .trait_import(Some("crate::ui::HasMutableVisibility"))
                .build().unwrap(),

            ImplDefBuilder::default()
                .trait_name("HasMutableZLevel")
                .trait_import(Some("crate::ui::HasMutableZLevel"))
                .build().unwrap()
        ])
        .methods(vec![

          MethodDefBuilder::default()
              .name("set_texture")
              .arguments(vec![
                ArgumentDefBuilder::default()
                    .name("texture")
                    .data_type(DataType::swift_generic(Some("T"),
                        DataType::swift_struct("Texture", None)))
                    .build().unwrap()
              ])
              .impl_block(Some(ImplBlockDefBuilder::default()
                  .trait_name("crate::ui::Sprite")
                  .build().unwrap()))
              .build().unwrap(),

            MethodDefBuilder::default()
                .name("animate")
                .arguments(vec![
                    ArgumentDefBuilder::default()
                        .name("animation")
                        .data_type(DataType::swift_generic(Some("A"),
                            DataType::swift_struct("Animation", None)))
                        .build().unwrap(),

                    ArgumentDefBuilder::default()
                        .name("frame_duration_sec")
                        .data_type(*DOUBLE)
                        .build().unwrap()
                ])
                .impl_block(Some(ImplBlockDefBuilder::default()
                    .trait_name("crate::ui::Sprite")
                    .build().unwrap()))
                .build().unwrap(),

            MethodDefBuilder::default()
                .name("clear_animations")
                .impl_block(Some(ImplBlockDefBuilder::default()
                    .trait_name("crate::ui::Sprite")
                    .build().unwrap()))
                .build().unwrap(),


          MethodDefBuilder::default()
              .name("remove_from_parent")
              .impl_block(Some(ImplBlockDefBuilder::default()
                  .trait_name("crate::ui::Sprite")
                  .build().unwrap()))
              .build().unwrap(),

          MethodDefBuilder::default()
              .name("set_size_animated")
              .arguments(vec![

                ArgumentDefBuilder::default()
                    .name("width")
                    .data_type(*DOUBLE)
                    .build().unwrap(),

                ArgumentDefBuilder::default()
                    .name("height")
                    .data_type(*DOUBLE)
                    .build().unwrap(),

                ArgumentDefBuilder::default()
                    .name("duration_seconds")
                    .data_type(*DOUBLE)
                    .build().unwrap()
              ])
              .impl_block(Some(ImplBlockDefBuilder::default()
                  .trait_name("HasMutableSize")
                  .build().unwrap()))
              .build().unwrap(),

          MethodDefBuilder::default()
              .name("set_location_animated")
              .arguments(vec![

                ArgumentDefBuilder::default()
                    .name("left")
                    .data_type(*DOUBLE)
                    .build().unwrap(),

                ArgumentDefBuilder::default()
                    .name("top")
                    .data_type(*DOUBLE)
                    .build().unwrap(),

                ArgumentDefBuilder::default()
                    .name("duration_seconds")
                    .data_type(*DOUBLE)
                    .build().unwrap()
              ])
              .impl_block(Some(ImplBlockDefBuilder::default()
                  .trait_name("HasMutableLocation")
                  .build().unwrap()))
              .build().unwrap(),

          MethodDefBuilder::default()
              .name("set_visible")
              .arguments(vec![
                ArgumentDefBuilder::default()
                    .name("visible")
                    .data_type(*BOOLEAN)
                    .build().unwrap()
              ])
              .impl_block(Some(ImplBlockDefBuilder::default()
                  .trait_name("HasMutableVisibility")
                  .build().unwrap()))
              .build().unwrap(),

            MethodDefBuilder::default()
                .name("set_color")
                .arguments(vec![
                ArgumentDefBuilder::default()
                    .name("color")
                    .data_type(*UINT)
                    .build().unwrap()
                ])
                .impl_block(Some(ImplBlockDefBuilder::default()
                    .trait_name("HasMutableColor")
                    .build().unwrap()))
                .build().unwrap(),

          MethodDefBuilder::default()
              .name("set_z_level")
              .arguments(vec![
                ArgumentDefBuilder::default()
                    .name("z_level")
                    .data_type(*DOUBLE)
                    .build().unwrap()
              ])
              .impl_block(Some(ImplBlockDefBuilder::default()
                  .trait_name("HasMutableZLevel")
                  .build().unwrap()))
              .build().unwrap()
        ])
        .custom_rust_drop_code(Some("crate::ui::Sprite::remove_from_parent(self);"))
        .build().unwrap(),

        TypeDefBuilder::default()
            .name("SpriteGroup")
            .rust_owned(false)
            .impls(vec![
                ImplDefBuilder::default()
                    .trait_name("crate::ui::SpriteGroup")
                    .trait_import(Some("crate::ui"))
                    .build().unwrap(),
                ImplDefBuilder::default()
                    .trait_name("crate::ui::SpriteSource")
                    .trait_import(Some("crate::ui"))
                    .generics(vec![
                        GenericDefBuilder::default()
                            .symbol(Some("T"))
                            .bound_type("Texture")
                            .build().unwrap(),
                        GenericDefBuilder::default()
                            .symbol(Some("S"))
                            .bound_type("Sprite")
                            .build().unwrap(),
                        GenericDefBuilder::default()
                            .symbol(Some("G"))
                            .bound_type("SpriteGroup")
                            .build().unwrap(),
                    ])
                    .build().unwrap(),
                ImplDefBuilder::default()
                    .trait_name("HasMutableZLevel")
                    .trait_import(Some("crate::ui::HasMutableZLevel"))
                    .build().unwrap(),
                ImplDefBuilder::default()
                    .trait_name("HasMutableVisibility")
                    .trait_import(Some("crate::ui::HasMutableVisibility"))
                    .build().unwrap()
            ])
            .methods(vec![
              MethodDefBuilder::default()
                  .name("set_z_level")
                  .arguments(vec![
                    ArgumentDefBuilder::default()
                        .name("z_level")
                        .data_type(*DOUBLE)
                        .build().unwrap()
                  ])
                  .impl_block(Some(ImplBlockDefBuilder::default()
                      .trait_name("HasMutableZLevel")
                      .build().unwrap()))
                  .build().unwrap(),
                MethodDefBuilder::default()
                    .name("set_visible")
                    .arguments(vec![
                    ArgumentDefBuilder::default()
                        .name("visible")
                        .data_type(*BOOLEAN)
                        .build().unwrap()
                    ])
                    .impl_block(Some(ImplBlockDefBuilder::default()
                        .trait_name("HasMutableVisibility")
                        .build().unwrap()))
                    .build().unwrap(),
            MethodDefBuilder::default()
                .name("remove_from_parent")
                .impl_block(Some(ImplBlockDefBuilder::default()
                    .trait_name("crate::ui::SpriteGroup")
                    .build().unwrap()))
                .build().unwrap(),
            MethodDefBuilder::default()
                .name("create_sprite")
                .impl_block(Some(ImplBlockDefBuilder::default()
                    .trait_name("crate::ui::SpriteSource")
                    .build().unwrap()))
                .return_type(Some(DataType::swift_generic(Some("S"),
                    DataType::swift_struct("Sprite", None))))
                .build().unwrap(),
            MethodDefBuilder::default()
                .name("create_group")
                .impl_block(Some(ImplBlockDefBuilder::default()
                    .trait_name("crate::ui::SpriteSource")
                    .build().unwrap()))
                .return_type(Some(DataType::swift_generic(Some("G"),
                    DataType::swift_struct("SpriteGroup", None))))
                .build().unwrap(),

            ])
            .custom_rust_drop_code(Some("crate::ui::SpriteGroup::remove_from_parent(self);"))
            .build().unwrap(),



    // Views

    TypeDefBuilder::default()
        .name("LoadingView")
        .rust_owned(false)
        .impls(vec![
            ImplDefBuilder::default()
                .trait_name("view::LoadingView")
                .trait_import(Some("crate::view"))
                .build().unwrap(),
            ImplDefBuilder::default()
                .trait_name("view::BaseView")
                .trait_import(Some("crate::view"))
                .build().unwrap()
        ])
        .methods(vec![
            MethodDefBuilder::default()
                .name("transition_to_main_menu_view")
                .impl_block(Some(ImplBlockDefBuilder::default()
                    .trait_name("view::LoadingView")
                    .build().unwrap()))
                .build().unwrap(),
            MethodDefBuilder::default()
                .name("initialize_pre_bind")
                .impl_block(Some(ImplBlockDefBuilder::default()
                    .trait_name("view::BaseView")
                    .build().unwrap()))
                .build().unwrap(),
            MethodDefBuilder::default()
                .name("initialize_post_bind")
                .impl_block(Some(ImplBlockDefBuilder::default()
                    .trait_name("view::BaseView")
                    .build().unwrap()))
                .arguments(vec![ArgumentDefBuilder::default()
                    .name("presenter")
                    .data_type(DataType::Any)
                    .build().unwrap()])
                .build().unwrap(),
        ])
        .build().unwrap(),

    TypeDefBuilder::default()
        .name("MainMenuView")
        .rust_owned(false)
        .impls(vec![
            ImplDefBuilder::default()
                .trait_name("view::MainMenuView")
                .trait_import(Some("crate::view"))
                .build().unwrap(),
            ImplDefBuilder::default()
                .trait_name("view::BaseView")
                .trait_import(Some("crate::view"))
                .build().unwrap()
        ])
        .methods(vec![
            MethodDefBuilder::default()
                .name("transition_to_game_view")
                .impl_block(Some(ImplBlockDefBuilder::default()
                    .trait_name("view::MainMenuView")
                    .build().unwrap()))
                .build().unwrap(),
            MethodDefBuilder::default()
                .name("initialize_pre_bind")
                .impl_block(Some(ImplBlockDefBuilder::default()
                    .trait_name("view::BaseView")
                    .build().unwrap()))
                .build().unwrap(),
            MethodDefBuilder::default()
                .name("initialize_post_bind")
                .impl_block(Some(ImplBlockDefBuilder::default()
                    .trait_name("view::BaseView")
                    .build().unwrap()))
                .arguments(vec![ArgumentDefBuilder::default()
                    .name("presenter")
                    .data_type(DataType::Any)
                    .build().unwrap()])
                .build().unwrap()
        ])
        .build().unwrap(),

    TypeDefBuilder::default()
        .name("GameView")
        .rust_owned(false)
        .impls(vec![
            ImplDefBuilder::default()
                .trait_name("view::GameView")
                .build().unwrap(),
            ImplDefBuilder::default()
                .trait_name("crate::ui::SpriteSource")
                .trait_import(Some("crate::ui"))
                .generics(vec![
                    GenericDefBuilder::default()
                        .symbol(Some("T"))
                        .bound_type("Texture")
                        .build().unwrap(),
                    GenericDefBuilder::default()
                        .symbol(Some("S"))
                        .bound_type("Sprite")
                        .build().unwrap(),
                    GenericDefBuilder::default()
                        .symbol(Some("G"))
                        .bound_type("SpriteGroup")
                        .build().unwrap(),
                ])
                .build().unwrap(),
            ImplDefBuilder::default()
                .trait_name("HasLayoutHandlers")
                .trait_import(Some("crate::ui::HasLayoutHandlers"))
                .generics(vec![
                    GenericDefBuilder::default()
                        .symbol(Some("R"))
                        .bound_type("HandlerRegistration")
                        .build().unwrap()
                ])
                .build().unwrap(),
            ImplDefBuilder::default()
                .trait_name("HasMagnifyHandlers")
                .trait_import(Some("crate::ui::HasMagnifyHandlers"))
                .generics(vec![
                    GenericDefBuilder::default()
                        .symbol(Some("R"))
                        .bound_type("HandlerRegistration")
                        .build().unwrap()
                ])
                .build().unwrap(),
            ImplDefBuilder::default()
                .trait_name("HasMultiTouchHandlers")
                .trait_import(Some("crate::ui::HasMultiTouchHandlers"))
                .generics(vec![
                    GenericDefBuilder::default()
                        .symbol(Some("R"))
                        .bound_type("HandlerRegistration")
                        .build().unwrap()
                ])
                .build().unwrap(),
            ImplDefBuilder::default()
                .trait_name("HasViewport")
                .trait_import(Some("crate::ui::HasViewport"))
                .generics(vec![
                    GenericDefBuilder::default()
                        .symbol(Some("V"))
                        .bound_type("Viewport")
                        .build().unwrap()
                ])
                .build().unwrap(),
            ImplDefBuilder::default()
                .trait_name("view::BaseView")
                .trait_import(Some("crate::view"))
                .build().unwrap()

        ])
        .fields(vec![
        ])
        .methods(vec![
            MethodDefBuilder::default()
                .name("add_multi_touch_handler")
                .impl_block(Some(ImplBlockDefBuilder::default()
                    .trait_name("HasMultiTouchHandlers")
                    .build().unwrap()))
                .arguments(vec![
                    ArgumentDefBuilder::default()
                        .name("multi_touch_handler")
                        .data_type(DataType::rust_struct(
                            "MultiTouchHandler",
                            Some("crate::ui::MultiTouchHandler")))
                        .build().unwrap()
                ])
                .return_type(Some(DataType::swift_generic(Some("R"),
                    DataType::swift_struct("HandlerRegistration", None))))
                .build().unwrap(),

            MethodDefBuilder::default()
                .name("add_layout_handler")
                .impl_block(Some(ImplBlockDefBuilder::default()
                    .trait_name("HasLayoutHandlers")
                    .build().unwrap()))
                .arguments(vec![
                    ArgumentDefBuilder::default()
                        .name("layout_handler")
                        .data_type(DataType::rust_struct(
                            "LayoutHandler",
                            Some("crate::ui::LayoutHandler")))
                        .build().unwrap()
                ])
                .return_type(Some(DataType::swift_generic(Some("R"),
                    DataType::swift_struct("HandlerRegistration", None))))
                .build().unwrap(),

            MethodDefBuilder::default()
                .name("add_magnify_handler")
                .impl_block(Some(ImplBlockDefBuilder::default()
                    .trait_name("HasMagnifyHandlers")
                    .build().unwrap()))
                .arguments(vec![
                    ArgumentDefBuilder::default()
                        .name("magnify_handler")
                        .data_type(DataType::rust_struct(
                            "MagnifyHandler",
                            Some("crate::ui::MagnifyHandler")))
                        .build().unwrap()
                ])
                .return_type(Some(DataType::swift_generic(Some("R"),
                    DataType::swift_struct("HandlerRegistration", None))))
                .build().unwrap(),

            MethodDefBuilder::default()
                .name("create_sprite")
                .impl_block(Some(ImplBlockDefBuilder::default()
                    .trait_name("crate::ui::SpriteSource")
                    .build().unwrap()))
                .return_type(Some(DataType::swift_generic(Some("S"),
                    DataType::swift_struct("Sprite", None))))
                .build().unwrap(),

            MethodDefBuilder::default()
                .name("create_group")
                .impl_block(Some(ImplBlockDefBuilder::default()
                    .trait_name("crate::ui::SpriteSource")
                    .build().unwrap()))
                .return_type(Some(DataType::swift_generic(Some("G"),
                    DataType::swift_struct("SpriteGroup", None))))
                .build().unwrap(),

            MethodDefBuilder::default()
                .name("get_viewport")
                .impl_block(Some(ImplBlockDefBuilder::default()
                    .trait_name("HasViewport")
                    .build().unwrap()))
                .return_type(Some(DataType::swift_generic(Some("V"),
                    DataType::swift_struct("Viewport", None))))
                .build().unwrap(),

            MethodDefBuilder::default()
                .name("initialize_pre_bind")
                .impl_block(Some(ImplBlockDefBuilder::default()
                    .trait_name("view::BaseView")
                    .build().unwrap()))
                .build().unwrap(),
            MethodDefBuilder::default()
                .name("initialize_post_bind")
                .impl_block(Some(ImplBlockDefBuilder::default()
                    .trait_name("view::BaseView")
                    .build().unwrap()))
                .arguments(vec![ArgumentDefBuilder::default()
                    .name("presenter")
                    .data_type(DataType::Any)
                    .build().unwrap()])
                .build().unwrap()
        ])
        .build().unwrap(),

    TypeDefBuilder::default()
        .name("Viewport")
        .rust_owned(false)
        .impls(vec![
            ImplDefBuilder::default()
                .trait_name("crate::ui::Viewport")
                .trait_import(Some("crate::ui"))
                .build().unwrap(),
            ImplDefBuilder::default()
                .trait_name("crate::ui::SpriteSource")
                .trait_import(Some("crate::ui"))
                .generics(vec![
                    GenericDefBuilder::default()
                        .symbol(Some("T"))
                        .bound_type("Texture")
                        .build().unwrap(),
                    GenericDefBuilder::default()
                        .symbol(Some("S"))
                        .bound_type("Sprite")
                        .build().unwrap(),
                    GenericDefBuilder::default()
                        .symbol(Some("G"))
                        .bound_type("SpriteGroup")
                        .build().unwrap(),
                ])
                .build().unwrap(),

            ImplDefBuilder::default()
                .trait_name("HasMutableScale")
                .trait_import(Some("crate::ui::HasMutableScale"))
                .build().unwrap(),

            ImplDefBuilder::default()
                .trait_name("HasMutableLocation")
                .trait_import(Some("crate::ui::HasMutableLocation"))
                .build().unwrap(),

            ImplDefBuilder::default()
                .trait_name("HasMutableVisibility")
                .trait_import(Some("crate::ui::HasMutableVisibility"))
                .build().unwrap()
        ])
        .methods(vec![
            MethodDefBuilder::default()
                .name("create_sprite")
                .impl_block(Some(ImplBlockDefBuilder::default()
                    .trait_name("crate::ui::SpriteSource")
                    .build().unwrap()))
                .return_type(Some(DataType::swift_generic(Some("S"),
                    DataType::swift_struct("Sprite", None))))
                .build().unwrap(),

            MethodDefBuilder::default()
            .name("create_group")
            .impl_block(Some(ImplBlockDefBuilder::default()
                .trait_name("crate::ui::SpriteSource")
                .build().unwrap()))
            .return_type(Some(DataType::swift_generic(Some("G"),
                DataType::swift_struct("SpriteGroup", None))))
            .build().unwrap(),

          MethodDefBuilder::default()
              .name("set_scale")
              .arguments(vec![

                ArgumentDefBuilder::default()
                    .name("scale")
                    .data_type(*DOUBLE)
                    .build().unwrap()
              ])
              .impl_block(Some(ImplBlockDefBuilder::default()
                  .trait_name("HasMutableScale")
                  .build().unwrap()))
              .build().unwrap(),

          MethodDefBuilder::default()
              .name("set_scale_and_location")
              .arguments(vec![

                ArgumentDefBuilder::default()
                    .name("scale")
                    .data_type(*DOUBLE)
                    .build().unwrap(),

                ArgumentDefBuilder::default()
                    .name("top_left_x")
                    .data_type(*DOUBLE)
                    .build().unwrap(),

                ArgumentDefBuilder::default()
                    .name("top_left_y")
                    .data_type(*DOUBLE)
                    .build().unwrap()
              ])
              .impl_block(Some(ImplBlockDefBuilder::default()
                  .trait_name("HasMutableScale")
                  .build().unwrap()))
              .build().unwrap(),

          MethodDefBuilder::default()
              .name("set_location_animated")
              .arguments(vec![

                ArgumentDefBuilder::default()
                    .name("left")
                    .data_type(*DOUBLE)
                    .build().unwrap(),

                ArgumentDefBuilder::default()
                    .name("top")
                    .data_type(*DOUBLE)
                    .build().unwrap(),

                ArgumentDefBuilder::default()
                    .name("duration_seconds")
                    .data_type(*DOUBLE)
                    .build().unwrap()
              ])
              .impl_block(Some(ImplBlockDefBuilder::default()
                  .trait_name("HasMutableLocation")
                  .build().unwrap()))
              .build().unwrap(),

          MethodDefBuilder::default()
              .name("set_visible")
              .arguments(vec![
                ArgumentDefBuilder::default()
                    .name("visible")
                    .data_type(*BOOLEAN)
                    .build().unwrap()
              ])
              .impl_block(Some(ImplBlockDefBuilder::default()
                  .trait_name("HasMutableVisibility")
                  .build().unwrap()))
              .build().unwrap()
            ])
        .build().unwrap(),

    // Native resources

    TypeDefBuilder::default()
        .name("SystemView")
        .rust_owned(false)
        .impls(vec![
            ImplDefBuilder::default()
                .trait_name("native::SystemView")
                .trait_import(Some("crate::native"))
                .generics(vec![
                    GenericDefBuilder::default()
                        .symbol(Some("T"))
                        .bound_type("Texture")
                        .build().unwrap(),

                    GenericDefBuilder::default()
                        .symbol(Some("TL"))
                        .bound_type("ResourceLoader")
                        .build().unwrap()
                ])
                .build().unwrap()
        ])
        .fields(vec![
            FieldDefBuilder::default()
                .name("resource_loader")
                .getter_impl(Some(ImplBlockDefBuilder::default()
                    .trait_name("native::SystemView")
                    .build().unwrap()))
                .data_type(DataType::swift_generic(Some("TL"),
                    DataType::swift_struct("ResourceLoader", None)))
                .build().unwrap()
        ])
        .build().unwrap(),

    TypeDefBuilder::default()
        .name("ResourceLoader")
        .rust_owned(false)
        .impls(vec![
            ImplDefBuilder::default()
                .trait_name("native::ResourceLoader")
                .trait_import(Some("crate::native"))
                .generics(vec![
                    GenericDefBuilder::default()
                        .symbol(Some("T"))
                        .bound_type("Texture")
                        .build().unwrap(),

                    GenericDefBuilder::default()
                        .symbol(Some("A"))
                        .bound_type("Animation")
                        .build().unwrap()
                ])
                .build().unwrap()
        ])
        .methods(vec![
            MethodDefBuilder::default()
                .name("load_texture")
                .impl_block(Some(ImplBlockDefBuilder::default()
                    .trait_name("native::ResourceLoader")
                    .build().unwrap()))
                .arguments(vec![
                    ArgumentDefBuilder::default()
                      .name("name")
                      .data_type(*STRING)
                      .build().unwrap()
                ])
                .return_type(Some(DataType::swift_generic(Some("T"),
                    DataType::swift_struct("Texture", None))))
                .build().unwrap(),

            MethodDefBuilder::default()
                .name("load_texture_from_png_data")
                .impl_block(Some(ImplBlockDefBuilder::default()
                    .trait_name("native::ResourceLoader")
                    .build().unwrap()))
                .arguments(vec![
                    ArgumentDefBuilder::default()
                    .name("png_data")
                    .data_type(TEXTURE_DATA.clone())
                    .build().unwrap()
                ])
                .return_type(Some(DataType::swift_generic(Some("T"),
                    DataType::swift_struct("Texture", None))))
                .build().unwrap(),

            MethodDefBuilder::default()
                .name("create_animation")
                .impl_block(Some(ImplBlockDefBuilder::default()
                    .trait_name("native::ResourceLoader")
                    .build().unwrap()))
                .return_type(Some(DataType::swift_generic(Some("A"),
                    DataType::swift_struct("Animation", None))))
                .build().unwrap(),

        ])
        .build().unwrap(),

  ];
}

handlebars_helper!(snake_case: |to_convert: str| {
  to_convert.to_snake_case()
});

handlebars_helper!(upper_case: |to_convert: str| {
  to_convert.to_uppercase()
});

handlebars_helper!(lower_camel: |to_convert: str| {
  to_convert.to_mixed_case()
});

pub fn generate() {
    info!("Generating swift bindings");

    info!("Building Handlebars");
    let mut hb = Handlebars::new();

    hb.register_escape_fn(|data| String::from(data));

    hb.register_helper("snake_case", Box::new(snake_case));
    hb.register_helper("upper_case", Box::new(upper_case));
    hb.register_helper("lower_camel", Box::new(lower_camel));

    hb.register_template_file(
        "rust_to_swift_binding",
        "build/templates/rust_to_swift_binding.handlebars",
    )
    .expect("Failed to load rust template");

    hb.register_template_file(
        "swift_to_rust_binding",
        "build/templates/swift_to_rust_binding.handlebars",
    )
    .expect("Failed to load swift template");

    hb.register_template_file(
        "c_header",
        "build/templates/c_header.handlebars",
    )
    .expect("Failed to load swift template");

    let mut rust_imports_set: BTreeSet<String> = BTreeSet::new();

    info!("Building Renderable Types");
    let renderable_types: Vec<RenderableType> = TYPES
        .iter()
        .map(|type_def| {
            info!("Building Renderable Types for {:?}", type_def.name);
            for import in type_def.get_all_imports() {
                rust_imports_set.insert(import);
            }
            type_def
        })
        .map(|type_def| RenderableType::from_def(&type_def))
        .collect();

    info!("Generating Imports");

    let mut rust_imports: Vec<String> = Vec::new();

    for import in rust_imports_set {
        rust_imports.push(import);
    }

    let mut renderable_context = RenderableContext {
        types: renderable_types,
        rust_imports,
        c_header_imports: String::default(),
        c_header_body: String::default(),
    };

    info!("Rendering Rust files");
    {
        // Render rust file
        let gen_path = Path::new("src");

        let rust_binding_file = gen_path.join(Path::new("lib_gen.rs"));

        let _ = fs::remove_file(&rust_binding_file);
        File::create(&rust_binding_file)
            .expect("Failed to create lib_swift_gen file");

        let mut options = OpenOptions::new();
        options.write(true);
        let writer: File = options.open(&rust_binding_file).unwrap();

        hb.render_to_write(
            "rust_to_swift_binding",
            &renderable_context,
            writer,
        )
        .expect("Failed to render swift_lib");
    }

    info!("Rendering Swift File");
    {
        // Render swift file
        let gen_path = Path::new("../enchantron-apple/Enchantron Shared");

        let rust_binding_file = gen_path.join(Path::new("RustBinder.swift"));

        let _ = fs::remove_file(&rust_binding_file);
        File::create(&rust_binding_file)
            .expect("Failed to create lib_swift file");

        let mut options = OpenOptions::new();
        options.write(true);
        let writer: File = options.open(&rust_binding_file).unwrap();

        hb.render_to_write(
            "swift_to_rust_binding",
            &renderable_context,
            writer,
        )
        .expect("Failed to render RustBinder");
    }

    // This is a kluge to make up for the fact that cbindgen can't handle
    // Rust 2018 :( ... Or something ... all I know is that the current
    // version of cbindgen (0.6.8) misses some structs
    info!("Rendering the c header");
    {
        let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

        let header_file_name = "enchantron.h";

        let mut cbindgen_contents_writer = StringWriter::new();

        cbindgen::Builder::new()
            .with_crate(crate_dir)
            .with_language(Language::C)
            .generate()
            .unwrap_or_else(|err| {
                error!("Unable to generate bindings {:?}", err);
                panic!("Failed to generate bindings")
            })
            .write(&mut cbindgen_contents_writer);

        let raw_cbindgen_contents = cbindgen_contents_writer.into_string();

        // Split the generated header into 2 on the first instance
        // of "typedef".  Everything above should be headers and constants, and
        // everything below should be types and methods
        let split_index =
            raw_cbindgen_contents.find("typedef").unwrap_or_else(|| {
                error!("Failed to find typedef in cbindgen content");
                panic!("Failed to find typedef in cbindgen content");
            });

        let (cbindgen_imports, cbindgen_body) =
            raw_cbindgen_contents.split_at(split_index);

        // Render swift file
        let gen_path = Path::new(".");

        let c_header_file = gen_path.join(Path::new(header_file_name));

        let _ = fs::remove_file(&c_header_file);
        File::create(&c_header_file).expect("Failed to create lib_swift file");

        let mut options = OpenOptions::new();
        options.write(true);
        let writer: File = options.open(&c_header_file).unwrap();

        renderable_context.c_header_imports = String::from(cbindgen_imports);
        renderable_context.c_header_body = String::from(cbindgen_body);

        hb.render_to_write("c_header", &renderable_context, writer)
            .expect("Failed to render c header");
    }

    info!("Finished Rendering Swift Bindings");
}
