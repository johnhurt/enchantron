use super::DynTraitDef;

#[derive(Clone, Debug, Serialize)]
pub struct RenderableDynTraitType {
    trait_name: String,
    trait_path: String,
}

impl RenderableDynTraitType {
    pub fn from_def(raw: &DynTraitDef) -> RenderableDynTraitType {
        RenderableDynTraitType {
            trait_name: raw.trait_name.to_owned(),
            trait_path: raw.trait_path.to_owned(),
        }
    }
}
