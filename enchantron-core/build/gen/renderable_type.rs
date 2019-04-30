use heck::SnakeCase;

use gen::{
    RenderableFunctionBuilder, RenderableImplBlock, RenderableImplBlockBuilder,
    RenderableWrappedType, TypeDef,
};

#[derive(Serialize, Builder, Clone, Default)]
#[builder(public)]
#[builder(default)]
#[builder(pattern = "owned")]
pub struct RenderableType {
    pub name: String,
    pub rust_owned: bool,
    pub impls: Vec<RenderableImplBlock>,
}

impl RenderableType {
    pub fn from_def(type_def: &TypeDef) -> RenderableType {
        RenderableTypeBuilder::default()
            .name(String::from(type_def.name))
            .rust_owned(type_def.rust_owned)
            .impls(type_def.get_renderable_functions())
            .build()
            .unwrap()
    }

    pub fn from_wrapped(
        wrapped_type: &RenderableWrappedType,
    ) -> RenderableType {
        RenderableTypeBuilder::default()
            .name(wrapped_type.wrapper_name.clone())
            .rust_owned(true)
            .impls(vec![RenderableImplBlockBuilder::default()
                .trait_name(Some(String::from("Drop")))
                .functions(vec![RenderableFunctionBuilder::default()
                    .name(format!(
                        "{}__drop",
                        wrapped_type.wrapper_name.to_snake_case()
                    ))
                    .impl_name(Some(String::from("drop")))
                    .type_name(wrapped_type.wrapper_name.clone())
                    .rust_owned(true)
                    .require_mutable_self(true)
                    .is_drop(true)
                    .build()
                    .unwrap()])
                .build()
                .unwrap()])
            .build()
            .unwrap()
    }
}
