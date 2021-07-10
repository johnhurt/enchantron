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
    ArgumentDefBuilder, GenericDefBuilder, ImplBlockDefBuilder, ImplDef,
    ImplDefBuilder, MethodDef, MethodDefBuilder, RenderableContext,
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

macro_rules! rust_struct {
    ($name:ident : $full_type:ty) => {
        DataType::rust_struct(stringify!($name), Some(stringify!($full_type)))
    };
    (Self::$generic_symbol:ident = $struct_name:ty) => {
        DataType::rust_generic(
            Some(stringify!($generic_symbol)),
            DataType::rust_struct(stringify!($struct_name), None),
            true,
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

macro_rules! methods {
    ($function_vec_name:ident = { }) => {
        $function_vec_name = vec![];
    };
    ($function_vec_name:ident = {$(
        $name:ident($($arg_name:ident : $arg_type_exp:expr),* $(,)? ) $( -> $return_exp:expr)?;
    )+}) => {
        $function_vec_name = vec![$(
            method!($name($($arg_name : $arg_type_exp),* ) $( -> $return_exp)?)
        ),*];
    }
}

macro_rules! one_impl {
    (impl $trait_name:ty => {
        $(
            type $associated_type_name:ident = $real_type:ty;
        )*
        $(
            fn $method_name:ident($($arg_name:ident : $arg_type_exp:expr),* $(,)? ) $( -> $return_exp:expr)? $( => $impl_method_body:block )?;
        )*
    }) => {{
        let impl_def = impl_def!($trait_name {$(
            type $associated_type_name = $real_type;
        )*});

        let impl_methods = vec![$(
            impl_method!($trait_name {
                fn $method_name($($arg_name : $arg_type_exp),*) $( -> $return_exp)? $( => $impl_method_body )?
            })
        ),*];

        (impl_def, impl_methods)
    }};
}

macro_rules! std_impl {
    (HasMutableSize) => {
        vec![one_impl!(impl crate::ui::HasMutableSize => {
            fn set_size_animated(
                width: DOUBLE,
                height: DOUBLE,
                duration_seconds: DOUBLE);
        })]
    };

    (HasSize) => {
        vec![one_impl!(impl crate::ui::HasSize => {
            fn get_width() -> DOUBLE;
            fn get_height() -> DOUBLE;
        })]
    };

    (HasText) => {
        vec![one_impl!(impl crate::ui::HasText => {
            fn get_text() -> STRING;
            fn set_text(text: STRING);
        })]
    };

    (HasMutableColor) => {
        vec![one_impl!(impl crate::ui::HasMutableColor => {
            type C = Color;

            fn set_color(color: UINT);
        })]
    };

    (HasMutableLocation) => {
        vec![one_impl!(impl crate::ui::HasMutableLocation => {
            fn set_location_animated(
                left: DOUBLE,
                top: DOUBLE,
                duration_seconds: DOUBLE);
        })]
    };

    (HasMutableVisibility) => {
        vec![one_impl!(impl crate::ui::HasMutableVisibility => {
            fn set_visible(visible: BOOLEAN);
        })]
    };

    (HasMutableZLevel) => {
        vec![one_impl!(impl crate::ui::HasMutableZLevel => {
            fn set_z_level(z_level: DOUBLE);
        })]
    };

    (SpriteSource) => {
        vec![one_impl!(impl crate::ui::SpriteSource => {
            type T = Texture;
            type S = Sprite;
            type G = SpriteGroup;

            fn create_sprite() -> swift_struct!(Self::S = Sprite);
            fn create_group() -> swift_struct!(Self::G = SpriteGroup);
        })]
    };

    (HasLayoutHandlers) => {
        vec![one_impl!(impl crate::ui::HasLayoutHandlers => {
            type R = HandlerRegistration;

            fn add_layout_handler(
                layout_handler: rust_struct!(LayoutHandler : crate::ui::LayoutHandler)
            ) -> swift_struct!(Self::R = HandlerRegistration);
        })]
    };

    (HasMultiTouchHandlers) => {
        vec![one_impl!(impl crate::ui::HasMultiTouchHandlers => {
            type R = HandlerRegistration;

            fn add_multi_touch_handler(
                multi_touch_handler: rust_struct!(MultiTouchHandler : crate::ui::MultiTouchHandler)
            ) -> swift_struct!(Self::R = HandlerRegistration);
        })]
    };

    (HasMagnifyHandlers) => {
        vec![one_impl!(impl crate::ui::HasMagnifyHandlers => {
            type R = HandlerRegistration;

            fn add_magnify_handler(
                magnify_handler: rust_struct!(MagnifyHandler : crate::ui::MagnifyHandler)
            ) -> swift_struct!(Self::R = HandlerRegistration);
        })]
    };

    (HasViewport) => {
        vec![one_impl!(impl crate::ui::HasViewport => {
            type V = Viewport;

            fn get_viewport() -> swift_struct!(Self::V = Viewport);
        })]
    };

    (HasMutableScale) => {
        vec![one_impl!(impl crate::ui::HasMutableScale => {
            fn set_scale(scale: DOUBLE);
            fn set_scale_and_location(
                scale: DOUBLE,
                top_left_x: DOUBLE,
                top_left_y: DOUBLE);
        })]
    };

}

macro_rules! std_impls {
    (($impls_vec_name:ident, $impl_funcs_vec_name:ident) = [$($std_trait:ident),*]) => {{

        let impl_and_methods_tuples : Vec<Vec<(ImplDef, Vec<MethodDef>)>> = vec![$(
            std_impl!($std_trait)
        ),*];

        $impls_vec_name = impl_and_methods_tuples
            .iter()
            .flatten()
            .map(|t| t.0.clone())
            .collect();

        $impl_funcs_vec_name = impl_and_methods_tuples
            .into_iter()
            .flatten()
            .map(|t| t.1)
            .collect();
    }}
}

macro_rules! impls {
    (($impls_vec_name:ident, $impl_funcs_vec_name:ident) = {}) => {
        $impls_vec_name = vec![];
        $impl_funcs_vec_name = vec![];
    };
    (($impls_vec_name:ident, $impl_funcs_vec_name:ident) = {$(
        impl $trait_name:ty => $impl_block:tt
    )+}) => {{

        let impl_and_methods_tuples  : Vec<(ImplDef, Vec<MethodDef>)> = vec![$(
            one_impl!(impl $trait_name => $impl_block)
        ),*];

        $impls_vec_name = impl_and_methods_tuples
            .iter()
            .map(|t| t.0.clone())
            .collect();

        $impl_funcs_vec_name = impl_and_methods_tuples
            .into_iter()
            .map(|t| t.1)
            .collect();
    }}
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
        fn $method_name:ident($(
            $arg_name:ident : $arg_type_exp:expr
        ),* $(,)? ) $( -> $return_exp:expr)? $( => $impl_method_body:block )?
    }) => {{
        let result = method_builder!($method_name($($arg_name: $arg_type_exp),*) $( -> $return_exp)?);
        let result = result.impl_block(Some(ImplBlockDefBuilder::default()
            .trait_name(stringify!($trait_name))
            .build().unwrap()));

        $(
            let result = result
                .custom_rust_code(Some(stringify!($impl_method_body)))
                .override_default_behavior(true);
        )?
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
    ($name:ident $(: $std_impl_1:ident $( + $std_impl_rest:ident )* )? {
        $(
            custom_rust_drop_code = $custom_rust_drop_code:expr;
        )?
        $(
            fn $method_name:ident($($arg_name:ident : $arg_type_exp:expr),* $(,)? ) $( -> $return_exp:expr)?;
        )*
        $(
            impl $trait_name:ty => $impl_block:tt
        )*
    }) => {{
        let regular_functions : Vec<MethodDef>;
        let impl_functions : Vec<Vec<MethodDef>>;
        let impl_blocks : Vec<ImplDef>;
        let std_impl_functions : Vec<Vec<MethodDef>>;
        let std_impl_blocks : Vec<ImplDef>;

        methods!(regular_functions = {$(
            $method_name($($arg_name:$arg_type_exp),*) $( -> $return_exp)?;
        )*});

        impls!((impl_blocks, impl_functions) = {$(
            impl $trait_name => $impl_block
        )*});

        std_impls!((std_impl_blocks, std_impl_functions)
            = [ $( $std_impl_1 $(, $std_impl_rest )* )? ]);

        let result = TypeDefBuilder::default()
            .name(stringify!($name))
            .methods(vec![
                regular_functions,
                impl_functions.into_iter().flatten().collect(),
                std_impl_functions.into_iter().flatten().collect()
            ].into_iter().flatten().collect())
            .impls(vec![
                impl_blocks,
                std_impl_blocks
            ].into_iter().flatten().collect());


        $(
            let result = result.custom_rust_drop_code(Some($custom_rust_drop_code));
        )?

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
            .name(stringify!($name))
            .empty_struct(true);

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

        fn get_length() -> LONG;

        fn get_content() -> MUTABLE_BYTE_POINTER;
    }),

    rust_type!(BoxedAny : crate::util::BoxedAny {}),

    // Application Root Object

    rust_type!( ApplicationContext {

        exclude_from_header = true;

        fn transition_to_loading_view();
        fn transition_to_main_menu_view(view: swift_struct!(NativeView));
        fn transition_to_game_view(view: swift_struct!(NativeView));
    }),

    // UI Components

    swift_type!( HandlerRegistration {
        custom_rust_drop_code = "crate::ui::HandlerRegistration::deregister(self);";

        impl crate::ui::HandlerRegistration => {
            fn deregister();
        }
    }),

    rust_type!( ClickHandler : crate::ui::ClickHandler {
        fn on_click();
    }),

    rust_type!( MagnifyHandler : crate::ui::MagnifyHandler {
        fn on_magnify(
            scale_change_additive: DOUBLE,
            zoom_center_x: DOUBLE,
            zoom_center_y: DOUBLE);
    }),

    rust_type!(MultiTouchHandler : crate::ui::MultiTouchHandler {
        fn on_one_drag_start(
            drag_id: LONG,
            global_x: DOUBLE,
            global_y: DOUBLE,
            click_count: LONG);
        fn on_one_drag_move(
            drag_id: LONG,
            global_x: DOUBLE,
            global_y: DOUBLE,
            click_count: LONG);
        fn on_one_drag_end(
            drag_id: LONG,
            global_x: DOUBLE,
            global_y: DOUBLE,
            click_count: LONG);

        fn on_two_drags_start(
            drag_id_1: LONG,
            global_x_1: DOUBLE,
            global_y_1: DOUBLE,
            click_count_1: LONG,
            drag_id_2: LONG,
            global_x_2: DOUBLE,
            global_y_2: DOUBLE,
            click_count_2: LONG);
        fn on_two_drags_move(
            drag_id_1: LONG,
            global_x_1: DOUBLE,
            global_y_1: DOUBLE,
            click_count_1: LONG,
            drag_id_2: LONG,
            global_x_2: DOUBLE,
            global_y_2: DOUBLE,
            click_count_2: LONG);
        fn on_two_drags_end(
            drag_id_1: LONG,
            global_x_1: DOUBLE,
            global_y_1: DOUBLE,
            click_count_1: LONG,
            drag_id_2: LONG,
            global_x_2: DOUBLE,
            global_y_2: DOUBLE,
            click_count_2: LONG);
    }),

    rust_type!(LayoutHandler : crate::ui::LayoutHandler {
        fn on_layout(width: DOUBLE, height: DOUBLE, scale: DOUBLE);
    }),

    empty_type!(ViewTypes {
        impl crate::view_types::ViewTypes {
            type Color = Color;
            type Sprite = Sprite;
            type SpriteGroup = SpriteGroup;
            type Texture = Texture;
            type Animation = Animation;
            type ResourceLoader = ResourceLoader;
            type SystemInterop = SystemInterop;
            type NativeView = NativeView;
            type ProgressBar = crate::ui::ProgressBarPublic<Self>;
            type Button = crate::ui::ButtonPublic<Self>;
            type LoadingView = crate::view::LoadingViewPublic<Self>;
            type MainMenuView = crate::view::MainMenuViewPublic<Self>;
            type GameView = crate::view::GameViewPublic<Self>;
            type Viewport = Viewport;
            type TransitionService = TransitionService;
        }
    }),

    swift_type!(TextArea : HasText {}),

    swift_type!(Animation {
        impl crate::native::Animation => {
            type Texture = Texture;

            fn add_texture(texture: swift_struct!( Self::Texture = Texture ));
            fn set_is_loop(is_loop: BOOLEAN);
            fn set_name(name: STRING);
        }
    }),

    swift_type!(Texture : HasSize {
        impl crate::native::Texture => {
            fn get_sub_texture(
                left: DOUBLE,
                top: DOUBLE,
                width: DOUBLE,
                weight: DOUBLE
            ) -> swift_struct!(Self = Texture);
        }
    }),

    swift_type!(Sprite : HasMutableSize + HasMutableColor + HasMutableLocation + HasMutableZLevel + HasMutableVisibility {
        custom_rust_drop_code = "crate::ui::Sprite::remove_from_parent(self);";

        impl crate::ui::Sprite => {
            type T = Texture;
            type A = Animation;

            fn set_texture(texture: swift_struct!(Self::T = Texture));
            fn animate(
                animation: swift_struct!(Self::A = Animation),
                frame_duration_sec: DOUBLE);
            fn clear_animations();
            fn remove_from_parent();
        }
    }),

    swift_type!(SpriteGroup : SpriteSource + HasMutableVisibility + HasMutableZLevel {
        custom_rust_drop_code = "crate::ui::SpriteGroup::remove_from_parent(self);";

        impl crate::ui::SpriteGroup => {
            fn remove_from_parent();
        }
    }),

    // Views

    swift_type!(NativeView : SpriteSource + HasLayoutHandlers + HasMagnifyHandlers + HasMultiTouchHandlers + HasViewport {
        impl crate::view::NativeView => {
            fn set_presenter(presenter: DataType::Any);
            fn unset_presenter();
        }
    }),

    swift_type!(Viewport : SpriteSource + HasMutableScale + HasMutableLocation + HasMutableVisibility {
        impl crate::ui::Viewport => {}
    }),

    swift_type!(TransitionService {
        impl crate::ui::TransitionService => {
            type NV = NativeView;
            type LV = crate::view::LoadingViewPublic<ViewTypes>;
            type MV = crate::view::MainMenuViewPublic<ViewTypes>;
            type GV = crate::view::GameViewPublic<ViewTypes>;

            fn transition_to_native_view(
                view : swift_struct!(Self::NV = NativeView),
                drop_current : BOOLEAN);

            fn transition_to_loading_view(
                view: rust_struct!(Self::LV = crate::view::LoadingViewPublic<ViewTypes>),
                drop_current: BOOLEAN
            ) => {
                self.transition_to_native_view(&view.inner.raw_view, drop_current)
            };

            fn transition_to_main_menu_view(
                view: rust_struct!(Self::MV = crate::view::MainMenuViewPublic<ViewTypes>),
                drop_current: BOOLEAN
            ) => {
                self.transition_to_native_view(&view.inner.raw_view, drop_current)
            };

            fn transition_to_game_view(
                view: rust_struct!(Self::GV = crate::view::GameViewPublic<ViewTypes>),
                drop_current: BOOLEAN
            ) => {
                self.transition_to_native_view(&view.inner.raw_view, drop_current)
            };
        }
    }),

    // Native resources

    swift_type!(SystemInterop {
        impl crate::native::SystemInterop => {
            type T = Texture;
            type TL = ResourceLoader;
            type TS = TransitionService;
            type NV = NativeView;
            type LV = crate::view::LoadingViewPublic<ViewTypes>;
            type MV = crate::view::MainMenuViewPublic<ViewTypes>;
            type GV = crate::view::GameViewPublic<ViewTypes>;

            fn get_resource_loader() -> swift_struct!(Self::TL = ResourceLoader);
            fn create_native_view() -> swift_struct!(Self::NV = NativeView);
            fn get_transition_service() -> swift_struct!(Self::TS = TransitionService);
            fn create_loading_view() -> rust_struct!(Self::LV = crate::view::LoadingViewPublic<ViewTypes>) => {
                crate::view::LoadingViewPublic::new(self.create_native_view())
            };
            fn create_main_menu_view() -> rust_struct!(Self::MV = crate::view::MainViewPublic<ViewTypes>) => {
                crate::view::MainMenuViewPublic::new(self.create_native_view())
            };
            fn create_game_view() -> rust_struct!(Self::GV = crate::view::GameViewPublic<ViewTypes>) => {
                crate::view::GameViewPublic::new(self.create_native_view())
            };
        }
    }),

    swift_type!(ResourceLoader {
        impl crate::native::ResourceLoader => {
            type T = Texture;
            type A = Animation;

            fn load_texture(name: STRING) -> swift_struct!(Self::T = Texture);
            fn load_texture_from_png_data(png_data: TEXTURE_DATA) -> swift_struct!(Self::T = Texture);
            fn create_animation() -> swift_struct!(Self::A = Animation);
        }
    }),


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
