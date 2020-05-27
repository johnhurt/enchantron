const FLOAT: &'static str = "FLOAT";
const VEC4_FLOAT: &'static str = "VEC4_FLOAT";

pub enum ShaderVariableType {
    Float,
    Vec4Float,
}

impl ShaderVariableType {
    pub fn type_name(&self) -> String {
        use self::*;

        match self {
            Float => FLOAT.to_owned(),
            Vec4Float => VEC4_FLOAT.to_owned(),
        }
    }
}
