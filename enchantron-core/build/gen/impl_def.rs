use gen::GenericDef;

#[derive(Serialize,Builder,Clone,Default)]
#[builder(public)]
#[builder(default)]
#[builder(pattern = "owned")]
pub struct ImplDef {
  pub trait_name: &'static str,
  pub trait_import: Option<&'static str>,
  pub generics: Vec<GenericDef>
}


impl ImplDef {
  pub fn get_imports(&self) -> Vec<String> {
    let mut result = Vec::new();

    if let Some(import) = self.trait_import {
      result.push(String::from(import))
    }

    result
  }
}