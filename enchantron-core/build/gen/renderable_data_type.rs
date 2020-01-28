use super::{
    ByteBufferType, DataType, RustStructDataType, SwiftGenericizedDataType,
    SwiftStructDataType,
};

#[derive(Serialize, Builder, Clone, Default)]
#[builder(public)]
#[builder(default)]
#[builder(pattern = "owned")]
pub struct RenderableDataType {
    pub name: String,
    pub sanitized_name: String,

    pub rust_name_internal: String,
    pub rust_name_incoming: String,
    pub rust_name_outgoing: String,
    pub borrow_outgoing: bool,

    pub rust_type_coersion_prefix_incoming: String,
    pub rust_type_coersion_postfix_incoming: String,
    pub rust_type_coersion_prefix_outgoing: String,
    pub rust_type_coersion_postfix_outgoing: String,

    pub swift_name_internal: String,
    pub swift_name_incoming: String,
    pub swift_name_outgoing: String,

    pub swift_type_coersion_prefix_incoming: String,
    pub swift_type_coersion_postfix_incoming: String,
    pub swift_type_coersion_prefix_outgoing: String,
    pub swift_type_coersion_postfix_outgoing: String,
}

impl RenderableDataType {
    pub fn from_raw(data_type: &DataType) -> RenderableDataType {
        let builder = RenderableDataTypeBuilder::default();

        match data_type {
            DataType::Nil => panic!("Nil data_type not valid ... yet"),
            DataType::Any => builder
                .name("Any".to_owned())
                .sanitized_name("Any".to_owned())
                .rust_name_internal("BoxedAny".to_owned())
                .rust_name_incoming("*mut BoxedAny".to_owned())
                .rust_name_outgoing("*mut BoxedAny".to_owned())
                .rust_type_coersion_prefix_incoming("unsafe { &*".to_owned())
                .rust_type_coersion_postfix_incoming(" }".to_owned())
                .rust_type_coersion_prefix_outgoing(
                    "Box::into_raw(Box::new(".to_owned(),
                )
                .rust_type_coersion_postfix_outgoing("))".to_owned())
                .swift_name_internal("RustAny".to_owned())
                .swift_name_incoming("OpaquePointer?".to_owned())
                .swift_name_outgoing("OpaquePointer?".to_owned())
                .swift_type_coersion_prefix_incoming("BoxedAny(".to_owned())
                .swift_type_coersion_postfix_incoming(")".to_owned())
                .swift_type_coersion_prefix_outgoing("".to_owned())
                .swift_type_coersion_postfix_outgoing(".ref".to_owned()),
            DataType::ByteBuffer(bb_type) => {
                render_byte_buffer_type(builder, bb_type)
            }
            DataType::Primitive(primitive_type) => builder
                .name(primitive_type.name.to_owned())
                .rust_name_internal(primitive_type.rust_name.to_owned())
                .rust_name_incoming(primitive_type.rust_name.to_owned())
                .rust_name_outgoing(primitive_type.rust_name.to_owned())
                .swift_name_internal(primitive_type.swift_name.to_owned())
                .swift_name_incoming(primitive_type.swift_name.to_owned())
                .swift_name_outgoing(primitive_type.swift_name.to_owned()),
            DataType::Future(struct_type) => builder
                .sanitized_name(format!("Future_{}", struct_type.name))
                .rust_name_internal(format!(
                    "Future<Output = {}>",
                    struct_type.name
                ))
                .rust_name_incoming(format!("*mut Future"))
                .rust_name_outgoing(format!("*mut Future_{}", struct_type.name))
                .rust_type_coersion_prefix_incoming("unsafe { &*".to_owned())
                .rust_type_coersion_postfix_incoming(" }".to_owned())
                .rust_type_coersion_prefix_outgoing(
                    "Box::into_raw(Box::new(".to_owned(),
                )
                .rust_type_coersion_postfix_outgoing("))".to_owned())
                .swift_name_internal(format!(
                    "RustFuture<{}>",
                    struct_type.name
                ))
                .swift_name_incoming("OpaquePointer?".to_owned())
                .swift_name_outgoing("OpaquePointer?".to_owned())
                .swift_type_coersion_prefix_incoming(
                    struct_type.name.to_owned() + &"(",
                )
                .swift_type_coersion_postfix_incoming(")".to_owned())
                .swift_type_coersion_prefix_outgoing("".to_owned())
                .swift_type_coersion_postfix_outgoing(".ref".to_owned()),
            DataType::RustGeneric(generic_type) => {
                render_rust_struct_type(&generic_type.bound_type, builder)
                    .rust_name_internal(
                        "Self".to_owned()
                            + &generic_type
                                .symbol
                                .map(|sym| format!("::{}", sym))
                                .unwrap_or("".to_owned()),
                    )
            }
            DataType::RustStruct(struct_type) => {
                render_rust_struct_type(struct_type, builder)
            }
            DataType::SwiftGeneric(generic_type) => {
                render_swift_struct_type(&generic_type.bound_type, builder)
                    .rust_name_internal(
                        "Self".to_owned()
                            + &generic_type
                                .symbol
                                .map(|sym| format!("::{}", sym))
                                .unwrap_or("".to_owned()),
                    )
            }
            DataType::SwiftStruct(struct_type) => {
                render_swift_struct_type(struct_type, builder)
            }
            DataType::SwiftGenericized(generic_type) => {
                render_swift_genericized_type(generic_type, builder)
            }
        }
        .build()
        .unwrap()
    }
}

fn render_rust_struct_type(
    struct_type: &RustStructDataType,
    builder: RenderableDataTypeBuilder,
) -> RenderableDataTypeBuilder {
    builder
        .sanitized_name(struct_type.name.to_owned())
        .rust_name_internal(struct_type.name.to_owned())
        .rust_name_incoming("*mut ".to_owned() + &struct_type.name)
        .rust_name_outgoing("*mut ".to_owned() + &struct_type.name)
        .rust_type_coersion_prefix_incoming("unsafe { &*".to_owned())
        .rust_type_coersion_postfix_incoming(" }".to_owned())
        .rust_type_coersion_prefix_outgoing(
            "Box::into_raw(Box::new(".to_owned(),
        )
        .rust_type_coersion_postfix_outgoing("))".to_owned())
        .swift_name_internal(struct_type.name.to_owned())
        .swift_name_incoming("OpaquePointer?".to_owned())
        .swift_name_outgoing("OpaquePointer?".to_owned())
        .swift_type_coersion_prefix_incoming(struct_type.name.to_owned() + &"(")
        .swift_type_coersion_postfix_incoming(")".to_owned())
        .swift_type_coersion_prefix_outgoing("".to_owned())
        .swift_type_coersion_postfix_outgoing(".ref".to_owned())
}

fn render_swift_struct_type(
    struct_type: &SwiftStructDataType,
    builder: RenderableDataTypeBuilder,
) -> RenderableDataTypeBuilder {
    builder
        .borrow_outgoing(true)
        .sanitized_name(struct_type.name.to_owned())
        .rust_name_internal(struct_type.name.to_owned())
        .rust_name_incoming(format!("*mut Opaque_{}", struct_type.name))
        .rust_name_outgoing(format!("*mut Opaque_{}", struct_type.name))
        .rust_type_coersion_prefix_incoming(struct_type.name.to_owned() + &"(")
        .rust_type_coersion_postfix_incoming(")".to_owned())
        .rust_type_coersion_prefix_outgoing("".to_owned())
        .rust_type_coersion_postfix_outgoing(".0".to_owned())
        .swift_name_internal(struct_type.name.to_owned())
        .swift_name_incoming("OpaquePointer?".to_owned())
        .swift_name_outgoing("OpaquePointer?".to_owned())
        .swift_type_coersion_prefix_incoming(
            "Unmanaged.fromOpaque(UnsafeRawPointer(".to_owned(),
        )
        .swift_type_coersion_postfix_incoming(
            "!)).takeUnretainedValue()".to_owned(),
        )
        .swift_type_coersion_prefix_outgoing(
            "OpaquePointer(Unmanaged.passRetained(".to_owned(),
        )
        .swift_type_coersion_postfix_outgoing(").toOpaque())".to_owned())
}

fn render_swift_genericized_type(
    generic_type: &SwiftGenericizedDataType,
    builder: RenderableDataTypeBuilder,
) -> RenderableDataTypeBuilder {
    render_swift_struct_type(&generic_type.bound_type, builder)
        .sanitized_name(generic_type.sanitized_name.to_owned())
        .rust_name_internal(format!("{}", generic_type.full_type))
        .rust_name_incoming(format!(
            "*mut Opaque_{}",
            generic_type.sanitized_name
        ))
        .rust_name_outgoing(format!(
            "*mut Opaque_{}",
            generic_type.sanitized_name
        ))
}

fn render_byte_buffer_type(
    builder: RenderableDataTypeBuilder,
    bb_type: &ByteBufferType,
) -> RenderableDataTypeBuilder {
    match bb_type {
        ByteBufferType::Stringy => builder
            .name("String".to_owned())
            .sanitized_name("String".to_owned())
            .rust_name_internal("String".to_owned())
            .rust_name_incoming("*mut Opaque_SwiftString".to_owned())
            .rust_name_outgoing("*mut ByteBuffer".to_owned())
            .rust_type_coersion_prefix_incoming("SwiftString(".to_owned())
            .rust_type_coersion_postfix_incoming(").to_string()".to_owned())
            .rust_type_coersion_prefix_outgoing(
                "Box::into_raw(Box::new(ByteBuffer::from_string(".to_owned(),
            )
            .rust_type_coersion_postfix_outgoing(")))".to_owned())
            .swift_name_internal("String".to_owned())
            .swift_name_incoming("OpaquePointer?".to_owned())
            .swift_name_outgoing("OpaquePointer?".to_owned())
            .swift_type_coersion_prefix_incoming(
                "byteBufferToString(ByteBuffer(".to_owned(),
            )
            .swift_type_coersion_postfix_incoming("))".to_owned())
            .swift_type_coersion_prefix_outgoing(
                "OpaquePointer(Unmanaged.passRetained(SwiftString(".to_owned(),
            )
            .swift_type_coersion_postfix_outgoing(")).toOpaque())".to_owned()),
        ByteBufferType::TextureData => builder
            .name("TextureData".to_owned())
            .sanitized_name("TextureData".to_owned())
            .rust_name_internal("ByteBuffer".to_owned())
            .rust_name_incoming("*mut Opaque_SwiftTextureData".to_owned())
            .rust_name_outgoing("*mut ByteBuffer".to_owned())
            .rust_type_coersion_prefix_incoming(
                "panic!(\"Cannot receive ".to_owned(),
            )
            .rust_type_coersion_postfix_incoming(" from swift\")".to_owned())
            .rust_type_coersion_prefix_outgoing(
                "Box::into_raw(Box::new(".to_owned(),
            )
            .rust_type_coersion_postfix_outgoing("))".to_owned())
            .swift_name_internal("String".to_owned())
            .swift_name_incoming("OpaquePointer?".to_owned())
            .swift_name_outgoing("OpaquePointer?".to_owned())
            .swift_type_coersion_prefix_incoming(
                "byteBufferToCGData(ByteBuffer(".to_owned(),
            )
            .swift_type_coersion_postfix_incoming("))".to_owned())
            .swift_type_coersion_prefix_outgoing(
                "OpaquePointer(Unmanaged.passRetained(SwiftString(".to_owned(),
            )
            .swift_type_coersion_postfix_outgoing(")).toOpaque())".to_owned()),
    }
}
