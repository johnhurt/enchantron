use itertools::Itertools;

lazy_static! {
    pub static ref LONG: DataType = DataType::Primitive(
        PrimitiveDataTypeBuilder::default()
            .name("Long")
            .rust_name("i64")
            .swift_name("Int64")
            .build()
            .unwrap()
    );
    pub static ref DOUBLE: DataType = DataType::Primitive(
        PrimitiveDataTypeBuilder::default()
            .name("Double")
            .rust_name("f64")
            .swift_name("Float64")
            .build()
            .unwrap()
    );
    pub static ref BOOLEAN: DataType = DataType::Primitive(
        PrimitiveDataTypeBuilder::default()
            .name("Boolean")
            .rust_name("bool")
            .swift_name("Bool")
            .build()
            .unwrap()
    );
    pub static ref MUTABLE_BYTE_POINTER: DataType = DataType::Primitive(
        PrimitiveDataTypeBuilder::default()
            .name("MutableBytePointer")
            .rust_name("*mut u8")
            .swift_name("UnsafeMutablePointer<UInt8>?")
            .build()
            .unwrap()
    );
    pub static ref STRING: DataType = DataType::Stringy;
}

#[derive(Serialize, Clone, Copy)]
pub enum DataType {
    Nil,
    Stringy,
    Primitive(PrimitiveDataType),
    Future(RustStructDataType),
    RustGeneric(RustGenericDataType),
    RustStruct(RustStructDataType),
    SwiftGeneric(SwiftGenericDataType),
    SwiftStruct(SwiftStructDataType),
    SwiftGenericized(SwiftGenericizedDataType),
}

#[derive(Serialize, Builder, Default, Clone, Copy)]
#[builder(default)]
#[builder(pattern = "owned")]
pub struct PrimitiveDataType {
    pub name: &'static str,
    pub rust_name: &'static str,
    pub swift_name: &'static str,
}

#[derive(Serialize, Builder, Default, Clone, Copy)]
#[builder(default)]
#[builder(public)]
#[builder(pattern = "owned")]
pub struct RustGenericDataType {
    pub symbol: Option<&'static str>,
    pub bound_type: RustStructDataType,
}

#[derive(Serialize, Builder, Default, Clone, Copy)]
#[builder(default)]
#[builder(public)]
#[builder(pattern = "owned")]
pub struct RustStructDataType {
    pub name: &'static str,
    pub import: Option<&'static str>,
}

#[derive(Serialize, Builder, Default, Clone, Copy)]
#[builder(default)]
#[builder(public)]
#[builder(pattern = "owned")]
pub struct SwiftGenericDataType {
    pub symbol: Option<&'static str>,
    pub bound_type: SwiftStructDataType,
}

#[derive(Serialize, Builder, Default, Clone, Copy)]
#[builder(default)]
#[builder(public)]
#[builder(pattern = "owned")]
pub struct SwiftGenericizedDataType {
    pub full_type: &'static str,
    pub sanitized_name: &'static str,
    pub bound_type: SwiftStructDataType,
}

#[derive(Serialize, Builder, Default, Clone, Copy)]
#[builder(default)]
#[builder(public)]
#[builder(pattern = "owned")]
pub struct SwiftStructDataType {
    pub name: &'static str,
    pub import: Option<&'static str>,
}

impl DataType {
    pub fn get_imports(&self) -> Vec<String> {
        match &self {
            DataType::Nil => Vec::new(),
            DataType::Any => vec![""],
            DataType::Stringy => vec![String::from("crate::util::RustString")],
            DataType::Primitive(_) => Vec::new(),
            DataType::Future(output_type) => {
                let mut result = output_type
                    .import
                    .iter()
                    .map(|import| import.to_string())
                    .collect_vec();

                result.push("std::future::Future".to_owned());
                result
            }
            DataType::RustGeneric(generic_type) => generic_type
                .bound_type
                .import
                .clone()
                .into_iter()
                .map(String::from)
                .collect(),
            DataType::SwiftGeneric(generic_type) => generic_type
                .bound_type
                .import
                .clone()
                .into_iter()
                .map(String::from)
                .collect(),
            DataType::SwiftStruct(_) => Vec::new(),
            DataType::RustStruct(struct_type) => struct_type
                .import
                .iter()
                .map(|import| import.to_string())
                .collect(),
            DataType::SwiftGenericized(generic_type) => generic_type
                .bound_type
                .import
                .clone()
                .into_iter()
                .map(String::from)
                .collect(),
        }
    }

    pub fn rust_generic(
        symbol: Option<&'static str>,
        bound_type: DataType,
    ) -> DataType {
        if let DataType::RustStruct(rust_type) = bound_type {
            DataType::RustGeneric(
                RustGenericDataTypeBuilder::default()
                    .symbol(symbol)
                    .bound_type(rust_type)
                    .build()
                    .unwrap(),
            )
        } else {
            panic!("Can only create a rust generic out of a rust struct")
        }
    }

    pub fn future(
        output_type_name: &'static str,
        import: Option<&'static str>,
    ) -> DataType {
        DataType::Future(
            RustStructDataTypeBuilder::default()
                .name(output_type_name)
                .import(import)
                .build()
                .unwrap(),
        )
    }

    pub fn rust_struct(
        name: &'static str,
        import: Option<&'static str>,
    ) -> DataType {
        DataType::RustStruct(
            RustStructDataTypeBuilder::default()
                .name(name)
                .import(import)
                .build()
                .unwrap(),
        )
    }

    pub fn swift_generic(
        symbol: Option<&'static str>,
        bound_type: DataType,
    ) -> DataType {
        if let DataType::SwiftStruct(swift_type) = bound_type {
            DataType::SwiftGeneric(
                SwiftGenericDataTypeBuilder::default()
                    .symbol(symbol)
                    .bound_type(swift_type)
                    .build()
                    .unwrap(),
            )
        } else {
            panic!("Can only create a swift generic out of a swift struct")
        }
    }

    pub fn swift_genericized(
        full_type: &'static str,
        sanitized_name: &'static str,
        bound_type: DataType,
    ) -> DataType {
        if let DataType::SwiftStruct(swift_type) = bound_type {
            DataType::SwiftGenericized(
                SwiftGenericizedDataTypeBuilder::default()
                    .full_type(full_type)
                    .sanitized_name(sanitized_name)
                    .bound_type(swift_type)
                    .build()
                    .unwrap(),
            )
        } else {
            panic!("Can only create a swift generic out of a swift struct")
        }
    }

    pub fn swift_struct(
        name: &'static str,
        import: Option<&'static str>,
    ) -> DataType {
        DataType::SwiftStruct(
            SwiftStructDataTypeBuilder::default()
                .name(name)
                .import(import)
                .build()
                .unwrap(),
        )
    }
}

impl Default for DataType {
    fn default() -> DataType {
        DataType::Nil
    }
}
