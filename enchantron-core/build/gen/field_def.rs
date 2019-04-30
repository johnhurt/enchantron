use gen::{
    DataType, ImplBlockDef, RenderableArgumentBuilder, RenderableDataType,
    RenderableFunction, RenderableFunctionBuilder, TypeDef,
};

use heck::SnakeCase;

#[derive(Default, Serialize, Builder, Clone)]
#[builder(public)]
#[builder(pattern = "owned")]
#[builder(default)]
pub struct FieldDef {
    pub name: &'static str,
    pub data_type: DataType,
    pub setter: bool,
    pub getter_impl: Option<ImplBlockDef>,
    pub setter_impl: Option<ImplBlockDef>,
}

impl FieldDef {
    pub fn create_getter(&self, type_def: &TypeDef) -> RenderableFunction {
        let mut impl_name = String::from("get_") + &String::from(self.name);

        if let Some(imp) = &self.getter_impl {
            if let Some(impl_method_name) = imp.impl_method_name {
                impl_name = String::from(impl_method_name);
            }
        }

        RenderableFunctionBuilder::default()
            .name(
                type_def.name.to_snake_case()
                    + &String::from("__get_")
                    + &self.name.to_snake_case(),
            )
            .type_name(String::from(type_def.name))
            .field_name(Some(String::from(self.name)))
            .is_getter(true)
            .impl_name(Some(impl_name))
            .return_type(Some(RenderableDataType::from_raw(&self.data_type)))
            .rust_owned(type_def.rust_owned)
            .build()
            .unwrap()
    }

    pub fn create_setter(&self, type_def: &TypeDef) -> RenderableFunction {
        let mut impl_name = String::from("set_") + &String::from(self.name);

        if let Some(imp) = &self.setter_impl {
            if let Some(impl_method_name) = imp.impl_method_name {
                impl_name = String::from(impl_method_name);
            }
        }

        RenderableFunctionBuilder::default()
            .name(
                type_def.name.to_snake_case()
                    + &String::from("__set_")
                    + &self.name.to_snake_case(),
            )
            .type_name(String::from(type_def.name))
            .impl_name(Some(impl_name))
            .field_name(Some(String::from(self.name)))
            .is_setter(true)
            .arguments(vec![RenderableArgumentBuilder::default()
                .name(String::from("value"))
                .data_type(RenderableDataType::from_raw(&self.data_type))
                .build()
                .unwrap()])
            .rust_owned(type_def.rust_owned)
            .build()
            .unwrap()
    }

    pub fn get_imports(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();

        result.append(&mut self.data_type.get_imports());

        result
    }
}
