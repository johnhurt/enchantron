use super::{RenderableDynTraitType, RenderableType};

#[derive(Serialize)]
pub struct RenderableContext {
    pub types: Vec<RenderableType>,
    pub rust_imports: Vec<String>,
    pub dyn_trait_types: Vec<RenderableDynTraitType>,
    pub c_header_imports: String,
    pub c_header_body: String,
}
