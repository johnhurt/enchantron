use gen::{RenderableDataType, ArgumentDef};

#[derive(Serialize,Builder,Clone, Default)]
#[builder(public)]
#[builder(default)]
#[builder(pattern = "owned")]
pub struct RenderableArgument {
  pub name: String,
  pub data_type: RenderableDataType
}

impl RenderableArgument {
  pub fn from_def(def: &ArgumentDef) -> RenderableArgument {
    RenderableArgumentBuilder::default()
        .name(String::from(def.name))
        .data_type(RenderableDataType::from_raw(&def.data_type))
        .build().unwrap()
  }
}