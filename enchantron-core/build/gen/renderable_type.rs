use gen::{ RenderableImplBlock, TypeDef, RenderableGeneric };

#[derive(Serialize,Builder,Clone, Default)]
#[builder(public)]
#[builder(default)]
#[builder(pattern = "owned")]
pub struct RenderableType {
  pub name: String,
  pub rust_owned: bool,
  pub rust_import: Option<String>,
  pub impls: Vec<RenderableImplBlock>
}

impl RenderableType {
  pub fn from_def(type_def: &TypeDef) -> RenderableType {
    RenderableTypeBuilder::default()
        .name(String::from(type_def.name))
        .rust_owned(type_def.rust_owned)
        .rust_import(type_def.rust_import.map(|import_name| {
              String::from(import_name)
            }))
        .impls(type_def.get_renderable_functions())
        .build().unwrap()
  }
}

