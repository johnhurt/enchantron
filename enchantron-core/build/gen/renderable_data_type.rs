use gen::{ DataType, RustStructDataType, SwiftStructDataType };

#[derive(Serialize,Builder,Clone, Default)]
#[builder(public)]
#[builder(default)]
#[builder(pattern = "owned")]
pub struct RenderableDataType {
  pub name: String,

  pub rust_name_internal: String,
  pub rust_name_incoming: String,
  pub rust_name_outgoing: String,

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
    let mut builder = RenderableDataTypeBuilder::default();

    match data_type {
      DataType::Nil => panic!("Nil data_type not valid ... yet"),
      DataType::Stringy => {
        builder
            .name(String::from("String"))
            .rust_name_internal(String::from("String"))
            .rust_name_incoming(String::from("*mut c_void"))
            .rust_name_outgoing(String::from("*mut RustString"))
            .rust_type_coersion_prefix_incoming(String::from("SwiftString("))
            .rust_type_coersion_postfix_incoming(String::from(").to_string()"))
            .rust_type_coersion_prefix_outgoing(String::from("Box::into_raw(Box::new(RustString::new("))
            .rust_type_coersion_postfix_outgoing(String::from(")))"))
            .swift_name_internal(String::from("String"))
            .swift_name_incoming(String::from("OpaquePointer?"))
            .swift_name_outgoing(String::from("UnsafeMutableRawPointer?"))
            .swift_type_coersion_prefix_incoming(String::from("rustStringToString(RustString("))
            .swift_type_coersion_postfix_incoming(String::from("))"))
            .swift_type_coersion_prefix_outgoing(String::from("UnsafeMutableRawPointer(Unmanaged.passRetained(SwiftString("))
            .swift_type_coersion_postfix_outgoing(String::from(")).toOpaque())"))
      },
      DataType::Primitive(primitive_type) => {
        builder
            .name(String::from(primitive_type.name))
            .rust_name_internal(String::from(primitive_type.rust_name))
            .rust_name_incoming(String::from(primitive_type.rust_name))
            .rust_name_outgoing(String::from(primitive_type.rust_name))
            .swift_name_internal(String::from(primitive_type.swift_name))
            .swift_name_incoming(String::from(primitive_type.swift_name))
            .swift_name_outgoing(String::from(primitive_type.swift_name))
      },
      DataType::RustGeneric(generic_type) => {
        render_rust_struct_type(&generic_type.bound_type, builder)
            .rust_name_internal(String::from("Self::")
                + &String::from(generic_type.symbol))
      },
      DataType::RustStruct(struct_type) => {
        render_rust_struct_type(struct_type, builder)
      },
      DataType::SwiftGeneric(generic_type) => {
        render_swift_struct_type(&generic_type.bound_type, builder)
            .rust_name_internal(String::from("Self::")
                + &String::from(generic_type.symbol))
      },
      DataType::SwiftStruct(struct_type) => {
        render_swift_struct_type(struct_type, builder)
      }
    }.build().unwrap()
  }

}

fn render_rust_struct_type(struct_type: &RustStructDataType
    , builder: RenderableDataTypeBuilder) -> RenderableDataTypeBuilder {
  builder
      .rust_name_internal(String::from(struct_type.name))
      .rust_name_incoming(String::from("*mut ")
          + &String::from(struct_type.name))
      .rust_name_outgoing(String::from("*mut ")
          + &String::from(struct_type.name))
      .rust_type_coersion_prefix_incoming(String::from("unsafe { &*"))
      .rust_type_coersion_postfix_incoming(String::from(" }"))
      .rust_type_coersion_prefix_outgoing(String::from("Box::into_raw(Box::new("))
      .rust_type_coersion_postfix_outgoing(String::from("))"))

      .swift_name_internal(String::from("struct_type.name"))
      .swift_name_incoming(String::from("OpaquePointer?"))
      .swift_name_outgoing(String::from("OpaquePointer?"))
      .swift_type_coersion_prefix_incoming(
          String::from(struct_type.name)
          + &String::from("("))
      .swift_type_coersion_postfix_incoming(String::from(")"))
      .swift_type_coersion_prefix_outgoing(String::from(""))
      .swift_type_coersion_postfix_outgoing(String::from(".ref"))
}

fn render_swift_struct_type(struct_type: &SwiftStructDataType
    , builder: RenderableDataTypeBuilder) -> RenderableDataTypeBuilder {
  builder
      .rust_name_internal(String::from(struct_type.name))
      .rust_name_incoming(String::from("*mut c_void"))
      .rust_name_outgoing(String::from("*mut c_void"))
      .rust_type_coersion_prefix_incoming(
          String::from(struct_type.name)
          + &String::from("("))
      .rust_type_coersion_postfix_incoming(String::from(")"))
      .rust_type_coersion_prefix_outgoing(String::from(""))
      .rust_type_coersion_postfix_outgoing(String::from(".0"))

      .swift_name_internal(String::from("struct_type.name"))
      .swift_name_incoming(String::from("UnsafeMutableRawPointer?"))
      .swift_name_outgoing(String::from("UnsafeMutableRawPointer?"))
      .swift_type_coersion_prefix_incoming(
          String::from("Unmanaged.fromOpaque(UnsafeRawPointer("))
      .swift_type_coersion_postfix_incoming(String::from("!))"))
      .swift_type_coersion_prefix_outgoing(
          String::from("UnsafeMutableRawPointer(Unmanaged.passRetained("))
      .swift_type_coersion_postfix_outgoing(String::from(").toOpaque())"))
}