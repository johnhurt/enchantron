use super::{ArgumentDef, DataType, ImplBlockDef};

#[derive(Default, Serialize, Builder, Clone)]
#[builder(public)]
#[builder(pattern = "owned")]
#[builder(default)]
pub struct MethodDef {
    pub name: &'static str,
    pub require_mutable_self: bool,
    pub rust_owned: bool,
    pub impl_block: Option<ImplBlockDef>,
    pub return_type: Option<DataType>,
    pub arguments: Vec<ArgumentDef>,
    pub custom_rust_code: Option<&'static str>,
}

impl MethodDef {
    pub fn get_imports(&self) -> Vec<String> {
        let mut result = Vec::new();

        if let Some(return_type) = self.return_type {
            result.append(&mut return_type.get_imports());
        }

        self.arguments
            .iter()
            .for_each(|arg| result.append(&mut arg.data_type.get_imports()));

        result
    }
}
