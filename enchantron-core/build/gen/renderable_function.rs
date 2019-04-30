use gen::{RenderableArgument, RenderableDataType};

#[derive(Serialize, Builder, Clone, Default)]
#[builder(public)]
#[builder(default)]
#[builder(pattern = "owned")]
pub struct RenderableFunction {
    pub name: String,
    pub type_name: String,
    pub impl_name: Option<String>,
    pub rust_owned: bool,
    pub is_drop: bool,
    pub is_setter: bool,
    pub is_getter: bool,
    pub field_name: Option<String>,
    pub require_mutable_self: bool,
    pub return_type: Option<RenderableDataType>,
    pub arguments: Vec<RenderableArgument>,
    pub custom_rust_code: Option<String>,
}
