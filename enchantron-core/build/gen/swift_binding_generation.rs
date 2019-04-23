use std::collections::BTreeSet;
use std::path::Path;
use std::fs::{OpenOptions, File};
use std::fs;

use heck::{ SnakeCase, MixedCase };

use handlebars::Handlebars;

use gen::{TypeDef, TypeDefBuilder, FieldDefBuilder,
        RenderableType, RenderableContext, MethodDefBuilder,
        ImplBlockDefBuilder, ImplDefBuilder, GenericDefBuilder,
        ArgumentDefBuilder, WrappedTypeDef, WrappedTypeDefBuilder,
        RenderableWrappedType };
use gen::data_type::*;

lazy_static!{
  static ref WRAPPED_TYPES : Vec<WrappedTypeDef> = vec![
    WrappedTypeDefBuilder::default()
        .wrapper_name("WrappedMainMenuPresenter")
        .wrapped_type_name("Arc<MainMenuPresenter<MainMenuView>>")
        .wrapped_type_imports(vec![
            "std::sync::Arc",
            "presenter::MainMenuPresenter"
            ])
        .build().unwrap(),
  ];

  #[derive(Serialize)]
  static ref TYPES : Vec<TypeDef> = vec![

    // Low-level Types

    TypeDefBuilder::default()
        .name("RustString")
        .rust_owned(true)
        .rust_import(Some("util::RustString"))
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
        .build().unwrap(),

    // UI Components

    TypeDefBuilder::default()
        .name("HandlerRegistration")
        .impls(vec![
            ImplDefBuilder::default()
                .trait_name("ui::HandlerRegistration")
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
        .rust_import(Some("ui::ClickHandler"))
        .rust_owned(true)
        .methods(vec![
            MethodDefBuilder::default()
                .name("on_click")
                .build().unwrap()
        ])
        .build().unwrap(),

    TypeDefBuilder::default()
        .name("Button")
        .rust_owned(false)
        .impls(vec![
            ImplDefBuilder::default()
                .trait_name("ui::Button")
                .build().unwrap(),

            ImplDefBuilder::default()
                .trait_name("HasText")
                .trait_import(Some("ui::HasText"))
                .build().unwrap(),

            ImplDefBuilder::default()
                .trait_name("HasClickHandlers")
                .trait_import(Some("ui::HasClickHandlers"))
                .generics(vec![
                    GenericDefBuilder::default()
                        .symbol("R")
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
                .return_type(Some(DataType::swift_generic("R",
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
                .trait_import(Some("ui::HasText"))
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


    // Views

    TypeDefBuilder::default()
        .name("MainMenuView")
        .rust_owned(false)
        .impls(vec![
            ImplDefBuilder::default()
                .trait_name("ui::MainMenuView")
                .generics(vec![
                    GenericDefBuilder::default()
                        .symbol("B")
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
                .data_type(DataType::swift_generic("B",
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

  let mut hb = Handlebars::new();

  hb.register_escape_fn(|data| {String::from(data) });

  hb.register_helper("snake_case", Box::new(snake_case));
  hb.register_helper("upper_case", Box::new(upper_case));
  hb.register_helper("lower_camel", Box::new(lower_camel));

  hb.register_template_file("rust_to_swift_binding",
      "build/templates/rust_to_swift_binding.handlebars")
          .expect("Failed to load rust template");

  hb.register_template_file("swift_to_rust_binding",
      "build/templates/swift_to_rust_binding.handlebars")
          .expect("Failed to load swift template");

  let mut rust_imports_set : BTreeSet<String> = BTreeSet::new();

  let mut renderable_types : Vec<RenderableType>
      = TYPES.iter()
        .map(|type_def| {
          for import in type_def.get_all_imports() {
            rust_imports_set.insert(import);
          }
          type_def
        })
        .map(|type_def| { RenderableType::from_def(&type_def) })
        .collect();

  let mut rust_imports : Vec<String> = Vec::new();

  for import in rust_imports_set {
    rust_imports.push(import);
  }

  let wrapped_types = WRAPPED_TYPES.iter()
      .map(|def| RenderableWrappedType::from_def(def))
      .collect();

  let renderable_context = RenderableContext {
    types: renderable_types,
    rust_imports: rust_imports,
    wrapped_types: wrapped_types
  };

  { // Render rust file
    let gen_path = Path::new("src");

    let rust_binding_file = gen_path.join(Path::new("lib_swift.rs"));

    let _ = fs::remove_file(&rust_binding_file);
    File::create(&rust_binding_file).expect("Failed to create lib_swift file");

    let mut options = OpenOptions::new();
    options.write(true);
    let writer : File = options.open(&rust_binding_file).unwrap();


    hb.render_to_write("rust_to_swift_binding", &renderable_context, writer)
        .expect("Failed to render swift_lib");

  }

  { // Render swift file
    let gen_path = Path::new("../enchantron-apple/Enchantron Shared");

    let rust_binding_file = gen_path.join(Path::new("RustBinder.swift"));

    let _ = fs::remove_file(&rust_binding_file);
    File::create(&rust_binding_file).expect("Failed to create lib_swift file");

    let mut options = OpenOptions::new();
    options.write(true);
    let writer : File = options.open(&rust_binding_file).unwrap();


    hb.render_to_write("swift_to_rust_binding", &renderable_context, writer)
        .expect("Failed to render RustBinder");

  }
}