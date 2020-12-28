use std::collections::HashMap;

use heck::SnakeCase;

use super::{
    DataType, ImplDef, MethodDef, RenderableArgument, RenderableDataType,
    RenderableFunctionBuilder, RenderableImplBlock, RenderableImplBlockBuilder,
};

#[derive(Serialize, Builder, Clone, Default)]
#[builder(public)]
#[builder(default)]
#[builder(pattern = "owned")]
pub struct TypeDef {
    pub name: &'static str,
    pub impls: Vec<ImplDef>,
    pub methods: Vec<MethodDef>,
    pub rust_owned: bool,
    pub empty_struct: bool,
    pub rust_import: Option<&'static str>,
    pub custom_rust_drop_code: Option<&'static str>,
    pub cloneable: bool,
    pub exclude_from_header: bool,
}

impl TypeDef {
    pub fn get_renderable_functions(&self) -> Vec<RenderableImplBlock> {
        let mut result: HashMap<String, RenderableImplBlock> = HashMap::new();

        let mut direct_impl = RenderableImplBlock::default();

        self.impls.iter().for_each(|imp| {
            result.insert(
                String::from(imp.trait_name),
                RenderableImplBlock::new_from_def(imp),
            );
        });

        for method_def in &self.methods {
            let method_func = RenderableFunctionBuilder::default()
                .name(
                    self.name.to_snake_case()
                        + &String::from("__")
                        + &method_def.name,
                ) // Should already be snake case
                .type_name(String::from(self.name))
                .require_mutable_self(method_def.require_mutable_self)
                .impl_name(Some(String::from(method_def.name)))
                .arguments(
                    method_def
                        .arguments
                        .iter()
                        .map(|arg_def| RenderableArgument::from_def(arg_def))
                        .collect(),
                )
                .return_type(
                    method_def
                        .return_type
                        .map(|rt_def| RenderableDataType::from_raw(&rt_def)),
                )
                .custom_rust_code(
                    method_def.custom_rust_code.map(|cc| String::from(cc)),
                )
                .override_default_behavior(method_def.override_default_behavior)
                .rust_owned(self.rust_owned)
                .build()
                .unwrap();

            if let Some(impl_block_def) = &method_def.impl_block {
                result
                    .get_mut(impl_block_def.trait_name)
                    .unwrap_or_else(|| {
                        error!(
                            "Trait {} is not an impl of struct {}",
                            impl_block_def.trait_name, self.name
                        );
                        panic!();
                    })
                    .functions
                    .push(method_func);
            } else {
                direct_impl.functions.push(method_func);
            }
        }

        // Empty Structs don't need drop impls
        if !self.empty_struct {
            result.insert(
                String::from("Drop"),
                RenderableImplBlockBuilder::default()
                    .trait_name(Some(String::from("Drop")))
                    .functions(vec![RenderableFunctionBuilder::default()
                        .name(
                            self.name.to_snake_case() + &String::from("__drop"),
                        )
                        .type_name(String::from(self.name))
                        .impl_name(Some(String::from("drop")))
                        .rust_owned(self.rust_owned)
                        .require_mutable_self(true)
                        .is_drop(true)
                        .custom_rust_code(
                            self.custom_rust_drop_code
                                .map(|code| String::from(code)),
                        )
                        .build()
                        .unwrap()])
                    .build()
                    .unwrap(),
            );
        }

        // Add a clone implementation for cloneable non-rust types
        if self.cloneable && !self.rust_owned {
            result.insert(
                "Clone".to_owned(),
                RenderableImplBlockBuilder::default()
                    .trait_name(Some("Clone".to_owned()))
                    .functions(vec![RenderableFunctionBuilder::default()
                        .name(
                            self.name.to_snake_case()
                                + &String::from("__clone"),
                        )
                        .return_type(Some(RenderableDataType::from_raw(
                            &DataType::swift_struct(self.name, None),
                        )))
                        .type_name(String::from(self.name))
                        .impl_name(Some(String::from("clone")))
                        .rust_owned(self.rust_owned)
                        .build()
                        .unwrap()])
                    .build()
                    .unwrap(),
            );
        }

        let mut result_vec: Vec<RenderableImplBlock> =
            result.drain().map(|(_, ib)| ib).collect();

        if !direct_impl.functions.is_empty() {
            result_vec.push(direct_impl);
        }

        result_vec
    }

    pub fn get_all_imports(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();

        if self.rust_owned {
            if let Some(self_import) = self.rust_import {
                result.push(String::from(self_import));
            }
        }

        for imp in &self.impls {
            if let Some(import) = imp.trait_import {
                result.push(String::from(import))
            }

            result.append(
                &mut imp
                    .generics
                    .iter()
                    .filter_map(|generic| {
                        generic
                            .bound_type_import
                            .map(|import| String::from(import))
                    })
                    .collect(),
            )
        }

        for method in &self.methods {
            result.append(&mut method.get_imports());
        }

        result
    }
}
