#[derive(Default, Serialize, Builder, Clone)]
#[builder(public)]
#[builder(pattern = "owned")]
#[builder(default)]
pub struct DynTraitDef {
    pub trait_name: &'static str,
    pub trait_path: &'static str,
    pub imports: Vec<&'static str>,
}
