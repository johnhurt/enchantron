use std::collections::BTreeSet;
use std::fs;
use std::fs::{File, OpenOptions};
use std::path::Path;

use heck::{MixedCase, SnakeCase};

use handlebars::{handlebars_helper, Handlebars};

use itertools::Itertools;

use super::data_type::*;
use super::{
    ArgumentDefBuilder, FieldDefBuilder, GenericDefBuilder,
    ImplBlockDefBuilder, ImplDefBuilder, MethodDefBuilder, RenderableContext,
    RenderableType, RenderableWrappedType, TypeDef, TypeDefBuilder,
    WrappedTypeDef, WrappedTypeDefBuilder,
};

lazy_static! {
  static ref WRAPPED_TYPES : Vec<WrappedTypeDef> = vec![
    WrappedTypeDefBuilder::default()
        .wrapper_name("WrappedMainMenuPresenter")
        .wrapped_type_name("Arc<MainMenuPresenter<MainMenuView>>")
        .wrapped_type_imports(vec![
            "std::sync::Arc",
            "crate::presenter::MainMenuPresenter"
        ])
        .build().unwrap(),

    WrappedTypeDefBuilder::default()
        .wrapper_name("WrappedLoadingPresenter")
        .wrapped_type_name("Arc<LoadingPresenter<LoadingView,SystemView>>")
        .wrapped_type_imports(vec![
            "std::sync::Arc",
            "crate::presenter::LoadingPresenter"
        ])
        .build().unwrap(),

    WrappedTypeDefBuilder::default()
        .wrapper_name("WrappedGamePresenter")
        .wrapped_type_name("Arc<GamePresenter<GameView,SystemView>>")
        .wrapped_type_imports(vec![
            "std::sync::Arc",
            "crate::presenter::GamePresenter"
        ])
        .build().unwrap(),
  ];

  #[derive(Serialize)]
  static ref TYPES : Vec<TypeDef> = vec![

    // Low-level Types

    TypeDefBuilder::default()
        .name("RustString")
        .rust_owned(true)
        .rust_import(Some("crate::util::RustString"))
        .methods(vec![

            MethodDefBuilder::default()
                .name("get_length")
                .return_type(Some(LONG.clone()))
                .build().unwrap(),

            MethodDefBuilder::default()
                .name("get_content")
                .return_type(Some(MUTABLE_BYTE_POINTER.clone()))
                .build().unwrap()
        ])
        .build().unwrap(),

    TypeDefBuilder::default()
        .name("SwiftString")
        .rust_owned(false)
        .fields(vec![
            FieldDefBuilder::default()
                .name("length")
                .data_type(LONG.clone())
                .setter(false)
                .build().unwrap()
        ])
        .methods(vec![
            MethodDefBuilder::default()
                .name("get_content")
                .return_type(Some(MUTABLE_BYTE_POINTER.clone()))
                .build().unwrap()
        ])
        .build().unwrap(),

    // Application Root Object

    TypeDefBuilder::default()
        .name("ApplicationContext")
        .rust_owned(true)
        .methods(vec![
          MethodDefBuilder::default()
              .name("bind_to_loading_view")
              .arguments(vec![
                ArgumentDefBuilder::default()
                    .name("view")
                    .data_type(DataType::swift_struct(
                        "LoadingView", None))
                    .build().unwrap()
              ])
              .return_type(Some(DataType::rust_struct(
                  "WrappedLoadingPresenter", None)))
              .build().unwrap(),

          MethodDefBuilder::default()
              .name("bind_to_main_menu_view")
              .arguments(vec![
                ArgumentDefBuilder::default()
                    .name("view")
                    .data_type(DataType::swift_struct(
                        "MainMenuView", None))
                    .build().unwrap()
              ])
              .return_type(Some(DataType::rust_struct(
                  "WrappedMainMenuPresenter", None)))
              .build().unwrap(),

          MethodDefBuilder::default()
              .name("bind_to_game_view")
              .arguments(vec![
                ArgumentDefBuilder::default()
                    .name("view")
                    .data_type(DataType::swift_struct(
                        "GameView", None))
                    .build().unwrap()
              ])
              .return_type(Some(DataType::rust_struct(
                  "WrappedGamePresenter", None)))
              .build().unwrap(),

        ])
        .build().unwrap(),

    // UI Components

    TypeDefBuilder::default()
        .name("HandlerRegistration")
        .impls(vec![
            ImplDefBuilder::default()
                .trait_name("ui::HandlerRegistration")
                .trait_import(Some("crate::ui"))
                .build().unwrap()
        ])
        .methods(vec![
            MethodDefBuilder::default()
                .name("deregister")
                .impl_block(Some(ImplBlockDefBuilder::default()
                    .trait_name("ui::HandlerRegistration")
                    .build().unwrap()))
                .build().unwrap()
        ])
        .rust_owned(false)
        .custom_rust_drop_code(Some("ui::HandlerRegistration::deregister(self);"))
        .build().unwrap(),

    TypeDefBuilder::default()
        .name("ClickHandler")
        .rust_import(Some("crate::ui::ClickHandler"))
        .rust_owned(true)
        .methods(vec![
            MethodDefBuilder::default()
                .name("on_click")
                .build().unwrap()
        ])
        .build().unwrap(),

    TypeDefBuilder::default()
        .name("DragHandler")
        .rust_import(Some("crate::ui::DragHandler"))
        .rust_owned(true)
        .methods(vec![
            MethodDefBuilder::default()
                .name("on_drag_start")
                .arguments(vec![
                  ArgumentDefBuilder::default()
                      .name("global_x")
                      .data_type(DOUBLE.clone())
                      .build().unwrap(),

                  ArgumentDefBuilder::default()
                      .name("global_y")
                      .data_type(DOUBLE.clone())
                      .build().unwrap(),

                  ArgumentDefBuilder::default()
                      .name("local_x")
                      .data_type(DOUBLE.clone())
                      .build().unwrap(),

                  ArgumentDefBuilder::default()
                      .name("local_y")
                      .data_type(DOUBLE.clone())
                      .build().unwrap(),
                ])
                .build().unwrap(),

            MethodDefBuilder::default()
                .name("on_drag_move")
                .arguments(vec![
                  ArgumentDefBuilder::default()
                      .name("global_x")
                      .data_type(DOUBLE.clone())
                      .build().unwrap(),

                  ArgumentDefBuilder::default()
                      .name("global_y")
                      .data_type(DOUBLE.clone())
                      .build().unwrap(),

                  ArgumentDefBuilder::default()
                      .name("local_x")
                      .data_type(DOUBLE.clone())
                      .build().unwrap(),

                  ArgumentDefBuilder::default()
                      .name("local_y")
                      .data_type(DOUBLE.clone())
                      .build().unwrap(),
                ])
                .build().unwrap(),

            MethodDefBuilder::default()
                .name("on_drag_end")
                .arguments(vec![
                  ArgumentDefBuilder::default()
                      .name("global_x")
                      .data_type(DOUBLE.clone())
                      .build().unwrap(),

                  ArgumentDefBuilder::default()
                      .name("global_y")
                      .data_type(DOUBLE.clone())
                      .build().unwrap(),

                  ArgumentDefBuilder::default()
                      .name("local_x")
                      .data_type(DOUBLE.clone())
                      .build().unwrap(),

                  ArgumentDefBuilder::default()
                      .name("local_y")
                      .data_type(DOUBLE.clone())
                      .build().unwrap(),
                ])
                .build().unwrap()
        ])
        .build().unwrap(),

    TypeDefBuilder::default()
        .name("LayoutHandler")
        .rust_import(Some("crate::ui::LayoutHandler"))
        .rust_owned(true)
        .methods(vec![
            MethodDefBuilder::default()
                .name("on_layout")
                .arguments(vec![
                  ArgumentDefBuilder::default()
                      .name("width")
                      .data_type(LONG.clone())
                      .build().unwrap(),
                  ArgumentDefBuilder::default()
                      .name("height")
                      .data_type(LONG.clone())
                      .build().unwrap()
                ])
                .build().unwrap()
        ])
        .build().unwrap(),

    TypeDefBuilder::default()
        .name("Button")
        .rust_owned(false)
        .impls(vec![
            ImplDefBuilder::default()
                .trait_name("crate::ui::Button")
                .trait_import(Some("crate::ui"))
                .build().unwrap(),

            ImplDefBuilder::default()
                .trait_name("HasText")
                .trait_import(Some("crate::ui::HasText"))
                .build().unwrap(),

            ImplDefBuilder::default()
                .trait_name("HasClickHandlers")
                .trait_import(Some("crate::ui::HasClickHandlers"))
                .generics(vec![
                    GenericDefBuilder::default()
                        .symbol(Some("R"))
                        .bound_type("HandlerRegistration")
                        .build().unwrap()
                ])
                .build().unwrap()
        ])
        .methods(vec![

            MethodDefBuilder::default()
                .name("get_text")
                .return_type(Some(STRING.clone()))
                .impl_block(Some(ImplBlockDefBuilder::default()
                    .trait_name("HasText")
                    .build().unwrap()))
                .build().unwrap(),

            MethodDefBuilder::default()
                .name("set_text")
                .arguments(vec![ArgumentDefBuilder::default()
                    .name("value")
                    .data_type(STRING.clone())
                    .build().unwrap()])
                .impl_block(Some(ImplBlockDefBuilder::default()
                    .trait_name("HasText")
                    .build().unwrap()))
                .build().unwrap(),

            MethodDefBuilder::default()
                .name("add_click_handler")
                .impl_block(Some(ImplBlockDefBuilder::default()
                    .trait_name("HasClickHandlers")
                    .build().unwrap()))
                .arguments(vec![
                    ArgumentDefBuilder::default()
                        .name("click_handler")
                        .data_type(DataType::rust_struct(
                            "ClickHandler",
                            Some("ui::ClickHandler")))
                        .build().unwrap()
                ])
                .return_type(Some(DataType::swift_generic(Some("R"),
                    DataType::swift_struct("HandlerRegistration", None))))
                .build().unwrap()
        ])
        .build().unwrap(),

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
                .data_type(STRING.clone())
                .setter(true)
                .build().unwrap()
        ])
        .build().unwrap(),

    TypeDefBuilder::default()
        .name("ProgressBar")
        .rust_owned(false)
        .impls(vec![
            ImplDefBuilder::default()
                .trait_name("HasText")
                .trait_import(Some("crate::ui::HasText"))
                .build().unwrap(),
            ImplDefBuilder::default()
                .trait_name("HasIntValue")
                .trait_import(Some("crate::ui::HasIntValue"))
                .build().unwrap(),
            ImplDefBuilder::default()
                .trait_name("ui::ProgressBar")
                .build().unwrap()
        ])
        .methods(vec![
            MethodDefBuilder::default()
                .name("get_text")
                .return_type(Some(STRING.clone()))
                .impl_block(Some(ImplBlockDefBuilder::default()
                    .trait_name("HasText")
                    .build().unwrap()))
                .build().unwrap(),

            MethodDefBuilder::default()
                .name("set_text")
                .arguments(vec![ArgumentDefBuilder::default()
                    .name("value")
                    .data_type(STRING.clone())
                    .build().unwrap()])
                .impl_block(Some(ImplBlockDefBuilder::default()
                    .trait_name("HasText")
                    .build().unwrap()))
                .build().unwrap(),

            MethodDefBuilder::default()
                .name("get_int_value")
                .return_type(Some(LONG.clone()))
                .impl_block(Some(ImplBlockDefBuilder::default()
                    .trait_name("HasIntValue")
                    .build().unwrap()))
                .build().unwrap(),

            MethodDefBuilder::default()
                .name("set_int_value")
                .arguments(vec![ArgumentDefBuilder::default()
                    .name("value")
                    .data_type(LONG.clone())
                    .build().unwrap()])
                .impl_block(Some(ImplBlockDefBuilder::default()
                    .trait_name("HasIntValue")
                    .build().unwrap()))
                .build().unwrap(),
        ])
        .build().unwrap(),

    TypeDefBuilder::default()
        .name("Texture")
        .rust_owned(false)
        .impls(vec![
            ImplDefBuilder::default()
                .trait_name("native::Texture")
                .trait_import(Some("crate::native"))
                .build().unwrap(),
            ImplDefBuilder::default()
                .trait_name("HasIntSize")
                .trait_import(Some("crate::native::HasIntSize"))
                .build().unwrap()
        ])
        .methods(vec![
            MethodDefBuilder::default()
                .name("get_width")
                .return_type(Some(LONG.clone()))
                .impl_block(Some(ImplBlockDefBuilder::default()
                    .trait_name("HasIntSize")
                    .build().unwrap()))
                .build().unwrap(),

            MethodDefBuilder::default()
                .name("get_height")
                .return_type(Some(LONG.clone()))
                .impl_block(Some(ImplBlockDefBuilder::default()
                    .trait_name("HasIntSize")
                    .build().unwrap()))
                .build().unwrap(),

            MethodDefBuilder::default()
                .name("get_sub_texture")
                .arguments(vec![
                    ArgumentDefBuilder::default()
                        .name("left")
                        .data_type(LONG.clone())
                        .build().unwrap(),
                    ArgumentDefBuilder::default()
                        .name("top")
                        .data_type(LONG.clone())
                        .build().unwrap(),
                    ArgumentDefBuilder::default()
                        .name("width")
                        .data_type(LONG.clone())
                        .build().unwrap(),
                    ArgumentDefBuilder::default()
                        .name("height")
                        .data_type(LONG.clone())
                        .build().unwrap()
                ])
                .impl_block(Some(ImplBlockDefBuilder::default()
                    .trait_name("native::Texture")
                    .build().unwrap()))
                .return_type(Some(DataType::swift_generic(None,
                    DataType::swift_struct("Texture", None))))
                .build().unwrap()
        ])
        .build().unwrap(),

    TypeDefBuilder::default()
        .name("Sprite")
        .rust_owned(false)
        .impls(vec![
            ImplDefBuilder::default()
                .trait_name("ui::Sprite")
                .trait_import(Some("crate::ui"))
                .generics(vec![
                  GenericDefBuilder::default()
                      .symbol(Some("T"))
                      .bound_type("Texture")
                      .build().unwrap()
                ])
                .build().unwrap(),

            ImplDefBuilder::default()
                .trait_name("HasDragHandlers")
                .trait_import(Some("crate::ui::HasDragHandlers"))
                .generics(vec![
                  GenericDefBuilder::default()
                      .symbol(Some("R"))
                      .bound_type("HandlerRegistration")
                      .build().unwrap()
                ])
                .build().unwrap(),

            ImplDefBuilder::default()
                .trait_name("HasMutableSize")
                .trait_import(Some("crate::ui::HasMutableSize"))
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
                .name("add_drag_handler")
                .impl_block(Some(ImplBlockDefBuilder::default()
                    .trait_name("HasDragHandlers")
                    .build().unwrap()))
                .arguments(vec![
                    ArgumentDefBuilder::default()
                        .name("drag_handler")
                        .data_type(DataType::rust_struct(
                            "DragHandler",
                            Some("ui::DragHandler")))
                        .build().unwrap()
                ])
                .return_type(Some(DataType::swift_generic(Some("R"),
                    DataType::swift_struct("HandlerRegistration", None))))
                .build().unwrap(),

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
                  .trait_name("ui::Sprite")
                  .build().unwrap()))
              .build().unwrap(),

          MethodDefBuilder::default()
              .name("propagate_events_to")
              .arguments(vec![
                ArgumentDefBuilder::default()
                    .name("sprite")
                    .data_type(DataType::swift_generic(None,
                        DataType::swift_struct("Sprite", None)))
                    .build().unwrap()
              ])
              .impl_block(Some(ImplBlockDefBuilder::default()
                  .trait_name("ui::Sprite")
                  .build().unwrap()))
              .build().unwrap(),

          MethodDefBuilder::default()
              .name("remove_from_parent")
              .impl_block(Some(ImplBlockDefBuilder::default()
                  .trait_name("ui::Sprite")
                  .build().unwrap()))
              .build().unwrap(),

          MethodDefBuilder::default()
              .name("set_size_animated")
              .arguments(vec![

                ArgumentDefBuilder::default()
                    .name("width")
                    .data_type(DOUBLE.clone())
                    .build().unwrap(),

                ArgumentDefBuilder::default()
                    .name("height")
                    .data_type(DOUBLE.clone())
                    .build().unwrap(),

                ArgumentDefBuilder::default()
                    .name("duration_seconds")
                    .data_type(DOUBLE.clone())
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
                    .data_type(DOUBLE.clone())
                    .build().unwrap(),

                ArgumentDefBuilder::default()
                    .name("top")
                    .data_type(DOUBLE.clone())
                    .build().unwrap(),

                ArgumentDefBuilder::default()
                    .name("duration_seconds")
                    .data_type(DOUBLE.clone())
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
                    .data_type(BOOLEAN.clone())
                    .build().unwrap()
              ])
              .impl_block(Some(ImplBlockDefBuilder::default()
                  .trait_name("HasMutableVisibility")
                  .build().unwrap()))
              .build().unwrap()
        ])
        .custom_rust_drop_code(Some("ui::Sprite::remove_from_parent(self);"))
        .build().unwrap(),

    // Views

    TypeDefBuilder::default()
        .name("LoadingView")
        .rust_owned(false)
        .impls(vec![
            ImplDefBuilder::default()
                .trait_name("ui::LoadingView")
                .trait_import(Some("crate::ui"))
                .generics(vec![
                    GenericDefBuilder::default()
                        .symbol(Some("P"))
                        .bound_type("ProgressBar")
                        .build().unwrap()
                ])
                .build().unwrap()
        ])
        .fields(vec![
            FieldDefBuilder::default()
                .name("progress_indicator")
                .getter_impl(Some(ImplBlockDefBuilder::default()
                    .trait_name("ui::LoadingView")
                    .build().unwrap()))
                .data_type(DataType::swift_generic(Some("P"),
                    DataType::swift_struct("ProgressBar", None)))
                .build().unwrap()
        ])
        .methods(vec![
            MethodDefBuilder::default()
                .name("transition_to_main_menu_view")
                .impl_block(Some(ImplBlockDefBuilder::default()
                    .trait_name("ui::LoadingView")
                    .build().unwrap()))
                .build().unwrap()
        ])
        .build().unwrap(),

    TypeDefBuilder::default()
        .name("MainMenuView")
        .rust_owned(false)
        .impls(vec![
            ImplDefBuilder::default()
                .trait_name("ui::MainMenuView")
                .trait_import(Some("crate::ui"))
                .generics(vec![
                    GenericDefBuilder::default()
                        .symbol(Some("B"))
                        .bound_type("Button")
                        .build().unwrap()
                ])
                .build().unwrap()
        ])
        .fields(vec![
            FieldDefBuilder::default()
                .name("start_new_game_button")
                .getter_impl(Some(ImplBlockDefBuilder::default()
                    .trait_name("ui::MainMenuView")
                    .build().unwrap()))
                .data_type(DataType::swift_generic(Some("B"),
                    DataType::swift_struct("Button", None)))
                .build().unwrap()
        ])
        .methods(vec![
            MethodDefBuilder::default()
                .name("transition_to_game_view")
                .impl_block(Some(ImplBlockDefBuilder::default()
                    .trait_name("ui::MainMenuView")
                    .build().unwrap()))
                .build().unwrap()
        ])
        .build().unwrap(),

    TypeDefBuilder::default()
        .name("GameView")
        .rust_owned(false)
        .impls(vec![
            ImplDefBuilder::default()
                .trait_name("ui::GameView")
                .build().unwrap(),
            ImplDefBuilder::default()
                .trait_name("ui::SpriteSource")
                .trait_import(Some("crate::ui"))
                .generics(vec![
                    GenericDefBuilder::default()
                        .symbol(Some("T"))
                        .bound_type("Texture")
                        .build().unwrap(),
                    GenericDefBuilder::default()
                        .symbol(Some("S"))
                        .bound_type("Sprite")
                        .build().unwrap()
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
                .trait_name("HasDragHandlers")
                .trait_import(Some("crate::ui::HasDragHandlers"))
                .generics(vec![
                    GenericDefBuilder::default()
                        .symbol(Some("R"))
                        .bound_type("HandlerRegistration")
                        .build().unwrap()
                ])
                .build().unwrap()
        ])
        .fields(vec![
            // FieldDefBuilder::default()
            //     .name("start_new_game_button")
            //     .getter_impl(Some(ImplBlockDefBuilder::default()
            //         .trait_name("ui::MainMenuView")
            //         .build().unwrap()))
            //     .data_type(DataType::swift_generic(Some("B"),
            //         DataType::swift_struct("Button", None)))
            //     .build().unwrap()
        ])
        .methods(vec![
            MethodDefBuilder::default()
                .name("add_drag_handler")
                .impl_block(Some(ImplBlockDefBuilder::default()
                    .trait_name("HasDragHandlers")
                    .build().unwrap()))
                .arguments(vec![
                    ArgumentDefBuilder::default()
                        .name("drag_handler")
                        .data_type(DataType::rust_struct(
                            "DragHandler",
                            Some("ui::DragHandler")))
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
                            Some("ui::LayoutHandler")))
                        .build().unwrap()
                ])
                .return_type(Some(DataType::swift_generic(Some("R"),
                    DataType::swift_struct("HandlerRegistration", None))))
                .build().unwrap(),

            MethodDefBuilder::default()
                .name("create_sprite")
                .impl_block(Some(ImplBlockDefBuilder::default()
                    .trait_name("ui::SpriteSource")
                    .build().unwrap()))
                .return_type(Some(DataType::swift_generic(Some("S"),
                    DataType::swift_struct("Sprite", None))))
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
                        .bound_type("TextureLoader")
                        .build().unwrap()
                ])
                .build().unwrap()
        ])
        .fields(vec![
            FieldDefBuilder::default()
                .name("texture_loader")
                .getter_impl(Some(ImplBlockDefBuilder::default()
                    .trait_name("native::SystemView")
                    .build().unwrap()))
                .data_type(DataType::swift_generic(Some("TL"),
                    DataType::swift_struct("TextureLoader", None)))
                .build().unwrap()
        ])
        .build().unwrap(),

    TypeDefBuilder::default()
        .name("TextureLoader")
        .rust_owned(false)
        .impls(vec![
            ImplDefBuilder::default()
                .trait_name("native::TextureLoader")
                .trait_import(Some("crate::native"))
                .generics(vec![
                    GenericDefBuilder::default()
                        .symbol(Some("T"))
                        .bound_type("Texture")
                        .build().unwrap()
                ])
                .build().unwrap()
        ])
        .methods(vec![
            MethodDefBuilder::default()
                .name("load_texture")
                .impl_block(Some(ImplBlockDefBuilder::default()
                    .trait_name("native::TextureLoader")
                    .build().unwrap()))
                .arguments(vec![
                    ArgumentDefBuilder::default()
                      .name("name")
                      .data_type(STRING.clone())
                      .build().unwrap()
                ])
                .return_type(Some(DataType::swift_generic(Some("T"),
                    DataType::swift_struct("Texture", None))))
                .build().unwrap()
        ])
        .build().unwrap()
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
    info!("Generating swift binindings");

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

    let mut rust_imports_set: BTreeSet<String> = BTreeSet::new();

    info!("Building Renderable Types");
    let mut renderable_types: Vec<RenderableType> = TYPES
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
    WRAPPED_TYPES
        .iter()
        .flat_map(|t| t.wrapped_type_imports.clone())
        .map(String::from)
        .for_each(|i| {
            rust_imports_set.insert(i);
        });

    let mut rust_imports: Vec<String> = Vec::new();

    for import in rust_imports_set {
        rust_imports.push(import);
    }

    info!("Building Wrapped Types");
    let wrapped_types: Vec<RenderableWrappedType> = WRAPPED_TYPES
        .iter()
        .map(RenderableWrappedType::from_def)
        .collect();

    renderable_types.append(
        &mut wrapped_types
            .iter()
            .map(RenderableType::from_wrapped)
            .collect(),
    );

    let renderable_context = RenderableContext {
        types: renderable_types,
        rust_imports: rust_imports,
        wrapped_types: wrapped_types,
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

    info!("Finished Rendering Swift Bingings");
}
