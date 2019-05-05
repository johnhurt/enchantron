use super::WrappedTypeDef;

#[derive(Serialize, Builder, Clone, Default)]
#[builder(public)]
#[builder(default)]
#[builder(pattern = "owned")]
pub struct RenderableWrappedType {
    pub wrapper_name: String,
    pub wrapped_type_name: String,
    pub wrapped_type_imports: Vec<String>,
}

impl RenderableWrappedType {
    pub fn from_def(def: &WrappedTypeDef) -> RenderableWrappedType {
        RenderableWrappedTypeBuilder::default()
            .wrapper_name(String::from(def.wrapper_name))
            .wrapped_type_name(String::from(def.wrapped_type_name))
            .wrapped_type_imports(
                def.wrapped_type_imports
                    .clone()
                    .into_iter()
                    .map(|import| String::from(import))
                    .collect(),
            )
            .build()
            .unwrap()
    }
}
