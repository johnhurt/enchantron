#[derive(Serialize, Builder, Clone, Default)]
#[builder(public)]
#[builder(default)]
#[builder(pattern = "owned")]
pub struct WrappedTypeDef {
    pub wrapper_name: &'static str,
    pub wrapped_type_name: &'static str,
    pub wrapped_type_imports: Vec<&'static str>,
}
