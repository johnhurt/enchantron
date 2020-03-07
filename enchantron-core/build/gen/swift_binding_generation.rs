use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::fs::{remove_file, File, OpenOptions};
use std::path::Path;

use cbindgen;
use cbindgen::Language;
use handlebars::{handlebars_helper, Handlebars, StringWriter};
use heck::{MixedCase, SnakeCase};
use itertools::Itertools;

use super::data_type::*;
use super::{
    ArgumentDefBuilder, FieldDefBuilder, GenericDefBuilder,
    ImplBlockDefBuilder, ImplDefBuilder, MethodDefBuilder, RenderableContext,
    RenderableType, TypeDef, TypeDefBuilder,
};

lazy_static! {

  #[derive(Serialize)]
  static ref TYPES : Vec<TypeDef> = vec![

    // Low-level Types

    TypeDefBuilder::default()
        .name("ByteBuffer")
        .rust_owned(true)
        .rust_import(Some("crate::util::ByteBuffer"))
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

    TypeDefBuilder::default()
            .name("BoxedAny")
            .rust_owned(true)
            .build().unwrap(),

    // Application Root Object

    TypeDefBuilder::default()
        .name("ApplicationContext")
        .rust_owned(true)
        .methods(vec![
          MethodDefBuilder::default()
              .name("transition_to_loading_view")
              .arguments(vec![
                ArgumentDefBuilder::default()
                    .name("view")
                    .data_type(DataType::swift_struct(
                        "LoadingView", None))
                    .build().unwrap()
              ])
              .return_type(None)
              .build().unwrap(),

          MethodDefBuilder::default()
              .name("transition_to_main_menu_view")
              .arguments(vec![
                ArgumentDefBuilder::default()
                    .name("view")
                    .data_type(DataType::swift_struct(
                        "MainMenuView", None))
                    .build().unwrap()
              ])
              .build().unwrap(),

          MethodDefBuilder::default()
              .name("transition_to_game_view")
              .arguments(vec![
                ArgumentDefBuilder::default()
                    .name("view")
                    .data_type(DataType::swift_struct(
                        "GameView", None))
                    .build().unwrap()
              ])
              .build().unwrap(),

        ])
        .build().unwrap(),

    // UI Components

    TypeDefBuilder::default()
        .name("HandlerRegistration")
        .impls(vec![
            ImplDefBuilder::default()
                .trait_name("crate::ui::HandlerRegistration")
                .trait_import(Some("crate::ui"))
                .build().unwrap()
        ])
        .methods(vec![
            MethodDefBuilder::default()
                .name("deregister")
                .impl_block(Some(ImplBlockDefBuilder::default()
                    .trait_name("crate::ui::HandlerRegistration")
                    .build().unwrap()))
                .build().unwrap()
        ])
        .rust_owned(false)
        .custom_rust_drop_code(Some("crate::ui::HandlerRegistration::deregister(self);"))
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
        .name("MagnifyHandler")
        .rust_import(Some("crate::ui::MagnifyHandler"))
        .rust_owned(true)
        .methods(vec![
            MethodDefBuilder::default()
                .name("on_magnify")
                .arguments(vec![
                    ArgumentDefBuilder::default()
                        .name("scale_change_additive")
                        .data_type(DOUBLE.clone())
                        .build().unwrap(),
                    ArgumentDefBuilder::default()
                        .name("zoom_center_x")
                        .data_type(DOUBLE.clone())
                        .build().unwrap(),
                    ArgumentDefBuilder::default()
                        .name("zoom_center_y")
                        .data_type(DOUBLE.clone())
                        .build().unwrap()
                ])
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
        .name("MultiDragHandler")
        .rust_import(Some("crate::ui::MultiDragHandler"))
        .rust_owned(true)
        .methods(vec![
            MethodDefBuilder::default()
                .name("on_one_drag_start")
                .arguments(vec![
                    ArgumentDefBuilder::default()
                        .name("drag_id")
                        .data_type(LONG.clone())
                        .build().unwrap(),

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
                .name("on_one_drag_move")
                .arguments(vec![
                    ArgumentDefBuilder::default()
                        .name("drag_id")
                        .data_type(LONG.clone())
                        .build().unwrap(),

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
                .name("on_one_drag_end")
                .arguments(vec![
                    ArgumentDefBuilder::default()
                        .name("drag_id")
                        .data_type(LONG.clone())
                        .build().unwrap(),

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
                    .name("on_two_drags_start")
                    .arguments(vec![
                        ArgumentDefBuilder::default()
                            .name("drag_id_1")
                            .data_type(LONG.clone())
                            .build().unwrap(),

                      ArgumentDefBuilder::default()
                          .name("global_x_1")
                          .data_type(DOUBLE.clone())
                          .build().unwrap(),

                      ArgumentDefBuilder::default()
                          .name("global_y_1")
                          .data_type(DOUBLE.clone())
                          .build().unwrap(),

                      ArgumentDefBuilder::default()
                          .name("local_x_1")
                          .data_type(DOUBLE.clone())
                          .build().unwrap(),

                      ArgumentDefBuilder::default()
                          .name("local_y_1")
                          .data_type(DOUBLE.clone())
                          .build().unwrap(),

                    ArgumentDefBuilder::default()
                        .name("drag_id_2")
                        .data_type(LONG.clone())
                        .build().unwrap(),

                    ArgumentDefBuilder::default()
                        .name("global_x_2")
                        .data_type(DOUBLE.clone())
                        .build().unwrap(),

                    ArgumentDefBuilder::default()
                        .name("global_y_2")
                        .data_type(DOUBLE.clone())
                        .build().unwrap(),

                    ArgumentDefBuilder::default()
                        .name("local_x_2")
                        .data_type(DOUBLE.clone())
                        .build().unwrap(),

                    ArgumentDefBuilder::default()
                        .name("local_y_2")
                        .data_type(DOUBLE.clone())
                        .build().unwrap(),
                    ])
                    .build().unwrap(),

                MethodDefBuilder::default()
                .name("on_two_drags_move")
                .arguments(vec![
                    ArgumentDefBuilder::default()
                        .name("drag_id_1")
                        .data_type(LONG.clone())
                        .build().unwrap(),

                  ArgumentDefBuilder::default()
                      .name("global_x_1")
                      .data_type(DOUBLE.clone())
                      .build().unwrap(),

                  ArgumentDefBuilder::default()
                      .name("global_y_1")
                      .data_type(DOUBLE.clone())
                      .build().unwrap(),

                  ArgumentDefBuilder::default()
                      .name("local_x_1")
                      .data_type(DOUBLE.clone())
                      .build().unwrap(),

                  ArgumentDefBuilder::default()
                      .name("local_y_1")
                      .data_type(DOUBLE.clone())
                      .build().unwrap(),

                ArgumentDefBuilder::default()
                    .name("drag_id_2")
                    .data_type(LONG.clone())
                    .build().unwrap(),

                ArgumentDefBuilder::default()
                    .name("global_x_2")
                    .data_type(DOUBLE.clone())
                    .build().unwrap(),

                ArgumentDefBuilder::default()
                    .name("global_y_2")
                    .data_type(DOUBLE.clone())
                    .build().unwrap(),

                ArgumentDefBuilder::default()
                    .name("local_x_2")
                    .data_type(DOUBLE.clone())
                    .build().unwrap(),

                ArgumentDefBuilder::default()
                    .name("local_y_2")
                    .data_type(DOUBLE.clone())
                    .build().unwrap(),
                ])
                .build().unwrap(),

                MethodDefBuilder::default()
                    .name("on_two_drags_end")
                    .arguments(vec![
                        ArgumentDefBuilder::default()
                            .name("drag_id_1")
                            .data_type(LONG.clone())
                            .build().unwrap(),

                      ArgumentDefBuilder::default()
                          .name("global_x_1")
                          .data_type(DOUBLE.clone())
                          .build().unwrap(),

                      ArgumentDefBuilder::default()
                          .name("global_y_1")
                          .data_type(DOUBLE.clone())
                          .build().unwrap(),

                      ArgumentDefBuilder::default()
                          .name("local_x_1")
                          .data_type(DOUBLE.clone())
                          .build().unwrap(),

                      ArgumentDefBuilder::default()
                          .name("local_y_1")
                          .data_type(DOUBLE.clone())
                          .build().unwrap(),

                    ArgumentDefBuilder::default()
                        .name("drag_id_2")
                        .data_type(LONG.clone())
                        .build().unwrap(),

                    ArgumentDefBuilder::default()
                        .name("global_x_2")
                        .data_type(DOUBLE.clone())
                        .build().unwrap(),

                    ArgumentDefBuilder::default()
                        .name("global_y_2")
                        .data_type(DOUBLE.clone())
                        .build().unwrap(),

                    ArgumentDefBuilder::default()
                        .name("local_x_2")
                        .data_type(DOUBLE.clone())
                        .build().unwrap(),

                    ArgumentDefBuilder::default()
                        .name("local_y_2")
                        .data_type(DOUBLE.clone())
                        .build().unwrap(),
                    ])
                    .build().unwrap(),
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
        .name("ViewTypes")
        .rust_owned(false)
        .empty_struct(true)
        .impls(vec![
            ImplDefBuilder::default()
                .trait_name("view_types::ViewTypes")
                .trait_import(Some("crate::view_types"))
                .generics(vec![
                    GenericDefBuilder::default()
                        .symbol(Some("Sprite"))
                        .bound_type("Sprite")
                        .build().unwrap(),

                    GenericDefBuilder::default()
                        .symbol(Some("SpriteGroup"))
                        .bound_type("SpriteGroup")
                        .build().unwrap(),

                    GenericDefBuilder::default()
                        .symbol(Some("ProgressBar"))
                        .bound_type("ProgressBar")
                        .build().unwrap(),

                    GenericDefBuilder::default()
                        .symbol(Some("LoadingView"))
                        .bound_type("LoadingView")
                        .build().unwrap(),

                    GenericDefBuilder::default()
                        .symbol(Some("Texture"))
                        .bound_type("Texture")
                        .build().unwrap(),

                    GenericDefBuilder::default()
                        .symbol(Some("TextureLoader"))
                        .bound_type("TextureLoader")
                        .build().unwrap(),

                    GenericDefBuilder::default()
                        .symbol(Some("SystemView"))
                        .bound_type("SystemView")
                        .build().unwrap(),

                    GenericDefBuilder::default()
                        .symbol(Some("GameView"))
                        .bound_type("GameView")
                        .build().unwrap(),

                    GenericDefBuilder::default()
                        .symbol(Some("Viewport"))
                        .bound_type("Viewport")
                        .build().unwrap(),

                    GenericDefBuilder::default()
                        .symbol(Some("MainMenuView"))
                        .bound_type("MainMenuView")
                        .build().unwrap(),

                    GenericDefBuilder::default()
                        .symbol(Some("Button"))
                        .bound_type("Button")
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
                            Some("crate::ui::ClickHandler")))
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
                .trait_name("crate::ui::ProgressBar")
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
                .trait_name("crate::ui::Sprite")
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
                .build().unwrap(),

            ImplDefBuilder::default()
                .trait_name("HasMutableZLevel")
                .trait_import(Some("crate::ui::HasMutableZLevel"))
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
                            Some("crate::ui::DragHandler")))
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
                  .trait_name("crate::ui::Sprite")
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
              .build().unwrap(),

          MethodDefBuilder::default()
              .name("set_z_level")
              .arguments(vec![
                ArgumentDefBuilder::default()
                    .name("z_level")
                    .data_type(DOUBLE.clone())
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
                        .data_type(DOUBLE.clone())
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
                        .data_type(BOOLEAN.clone())
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
                .generics(vec![
                    GenericDefBuilder::default()
                        .symbol(Some("P"))
                        .bound_type("ProgressBar")
                        .build().unwrap()
                ])
                .build().unwrap(),
            ImplDefBuilder::default()
                .trait_name("view::BaseView")
                .trait_import(Some("crate::view"))
                .build().unwrap()
        ])
        .fields(vec![
            FieldDefBuilder::default()
                .name("progress_indicator")
                .getter_impl(Some(ImplBlockDefBuilder::default()
                    .trait_name("view::LoadingView")
                    .build().unwrap()))
                .data_type(DataType::swift_generic(Some("P"),
                    DataType::swift_struct("ProgressBar", None)))
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
                .generics(vec![
                    GenericDefBuilder::default()
                        .symbol(Some("B"))
                        .bound_type("Button")
                        .build().unwrap()
                ])
                .build().unwrap(),
            ImplDefBuilder::default()
                .trait_name("view::BaseView")
                .trait_import(Some("crate::view"))
                .build().unwrap()
        ])
        .fields(vec![
            FieldDefBuilder::default()
                .name("start_new_game_button")
                .getter_impl(Some(ImplBlockDefBuilder::default()
                    .trait_name("view::MainMenuView")
                    .build().unwrap()))
                .data_type(DataType::swift_generic(Some("B"),
                    DataType::swift_struct("Button", None)))
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
                .trait_name("HasMultiDragHandlers")
                .trait_import(Some("crate::ui::HasMultiDragHandlers"))
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
                .name("add_multi_drag_handler")
                .impl_block(Some(ImplBlockDefBuilder::default()
                    .trait_name("HasMultiDragHandlers")
                    .build().unwrap()))
                .arguments(vec![
                    ArgumentDefBuilder::default()
                        .name("multi_drag_handler")
                        .data_type(DataType::rust_struct(
                            "MultiDragHandler",
                            Some("crate::ui::MultiDragHandler")))
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
                    .data_type(DOUBLE.clone())
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
                    .data_type(DOUBLE.clone())
                    .build().unwrap(),

                ArgumentDefBuilder::default()
                    .name("top_left_x")
                    .data_type(DOUBLE.clone())
                    .build().unwrap(),

                ArgumentDefBuilder::default()
                    .name("top_left_y")
                    .data_type(DOUBLE.clone())
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
                .build().unwrap(),

            MethodDefBuilder::default()
            .name("load_texture_from_png_data")
            .impl_block(Some(ImplBlockDefBuilder::default()
                .trait_name("native::TextureLoader")
                .build().unwrap()))
            .arguments(vec![
                ArgumentDefBuilder::default()
                  .name("png_data")
                  .data_type(TEXTURE_DATA.clone())
                  .build().unwrap()
            ])
            .return_type(Some(DataType::swift_generic(Some("T"),
                DataType::swift_struct("Texture", None))))
            .build().unwrap()
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

    hb.register_template_file(
        "c_header",
        "build/templates/c_header.handlebars",
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

        // Split the generated header into 2 on the first insteance
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

    info!("Finished Rendering Swift Bingings");
}
