
#[derive(Serialize,Builder,Clone,Default)]
#[builder(public)]
#[builder(default)]
#[builder(pattern = "owned")]
pub struct GenericDef {
  pub symbol: Option<&'static str>,
  pub bound_type: &'static str,
  pub bound_type_import: Option<&'static str>
}
