use super::{RenderableType, RenderableWrappedType};

#[derive(Serialize)]
pub struct RenderableContext {
    pub types: Vec<RenderableType>,
    pub rust_imports: Vec<String>,
    pub wrapped_types: Vec<RenderableWrappedType>,
    pub c_header_imports: String,
    pub c_header_body: String,
}
