use gen::{RenderableFunction, ImplDef, RenderableGeneric};

#[derive(Serialize,Builder,Clone,Default)]
#[builder(public)]
#[builder(default)]
#[builder(pattern = "owned")]
pub struct RenderableImplBlock {
  pub trait_name: Option<String>,
  pub functions: Vec<RenderableFunction>,
  pub generics: Vec<RenderableGeneric>
}

impl RenderableImplBlock {
  pub fn new_from_def(def: &ImplDef) -> RenderableImplBlock {
    RenderableImplBlockBuilder::default()
        .trait_name(Some(String::from(def.trait_name)))
        .generics(def.generics.iter().map(|generic| {
              RenderableGeneric::from_def(&generic)
            }).collect())
        .build()
        .unwrap()
  }
}