use gen::{RenderableType, RenderableWrappedType};

#[derive(Serialize)]
pub struct RenderableContext {
    pub types: Vec<RenderableType>,
    pub rust_imports: Vec<String>,
    pub wrapped_types: Vec<RenderableWrappedType>,
}
