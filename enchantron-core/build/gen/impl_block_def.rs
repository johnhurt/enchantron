
#[derive(Serialize,Builder,Clone,Default)]
#[builder(public)]
#[builder(default)]
#[builder(pattern = "owned")]
pub struct ImplBlockDef {
  pub trait_name: &'static str,
  pub impl_method_name: Option<&'static str>,
}
