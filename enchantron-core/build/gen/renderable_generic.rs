use gen::GenericDef;

#[derive(Serialize,Builder,Clone,Default)]
#[builder(public)]
#[builder(default)]
#[builder(pattern = "owned")]
pub struct RenderableGeneric {
  pub symbol: String,
  pub bound_type: String
}

impl RenderableGeneric {
  pub fn from_def(def: &GenericDef) -> RenderableGeneric {
    RenderableGenericBuilder::default()
      .symbol(String::from(def.symbol))
      .bound_type(String::from(def.bound_type))
      .build().unwrap()
  }
}