const FLOAT: &'static str = "FLOAT";
const VEC2_FLOAT: &'static str = "VEC2_FLOAT";
const VEC3_FLOAT: &'static str = "VEC3_FLOAT";
const VEC4_FLOAT: &'static str = "VEC4_FLOAT";

pub enum ShaderVariableType {
    Float,
    Vec2Float,
    Vec3Float,
    Vec4Float,
}

impl ShaderVariableType {
    pub fn type_name(&self) -> String {
        use self::*;

        match self {
            Float => FLOAT.to_owned(),
            Vec2Float => VEC2_FLOAT.to_owned(),
            Vec3Float => VEC3_FLOAT.to_owned(),
            Vec4Float => VEC4_FLOAT.to_owned(),
        }
    }
}
