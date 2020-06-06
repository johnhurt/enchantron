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
        match self {
            Self::Float => FLOAT.to_owned(),
            Self::Vec2Float => VEC2_FLOAT.to_owned(),
            Self::Vec3Float => VEC3_FLOAT.to_owned(),
            Self::Vec4Float => VEC4_FLOAT.to_owned(),
        }
    }
}

#[test]
fn test_sanity() {
    assert_eq!(
        ShaderVariableType::Vec4Float.type_name(),
        VEC4_FLOAT.to_owned()
    );
}
