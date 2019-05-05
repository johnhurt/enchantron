use super::DataType;

#[derive(Default, Serialize, Builder, Clone)]
#[builder(public)]
#[builder(pattern = "owned")]
#[builder(default)]
pub struct ArgumentDef {
    pub name: &'static str,
    pub data_type: DataType,
}
