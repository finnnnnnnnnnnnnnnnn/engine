use std::collections::btree_set::Union;
use std::fs;
use codegen::Scope;
use weedle::{interface::OperationInterfaceMember, Definition};
use weedle::types::{NonAnyType as NAType, ReturnType as RType, SingleType as SType, Type as WType, UnionMemberType};

fn main() {
    let widl = fs::read_to_string("test.widl").unwrap();
    let parsed = weedle::parse(&widl).unwrap();
    let mut scope = Scope::new();

    let interface_definition = parsed.iter().find_map(|d| match d {
        Definition::Interface(interface) => Some(interface.to_owned()),
        _=> None
    }).expect("No interface found");
    let interface = InterfaceConfig::new(interface_definition);
    println!("{:#?}", interface);
}


#[derive(Debug)]
struct InterfaceMethod {
    name: String,
    return_type: String,
    params: Vec<InterfaceMethodParam>
}

#[derive(Debug, Clone)]
struct InterfaceField {
    field_name: String,
    field_type: String
}

#[derive(Debug, Clone)]
struct InterfaceStruct {
    name: String,
    fields: Vec<InterfaceField>
}
#[derive(Debug)]
struct InterfaceImpl {
    methods: Vec<InterfaceMethod>
}
#[derive(Debug)]
struct InterfaceEnum {}
#[derive(Debug)]
struct InterfaceConfig {
    interface_struct: InterfaceStruct,
    interface_impl: InterfaceImpl,
    interface_enums: Vec<UnionTypeEnum>
}


impl InterfaceConfig {
    fn new(interface: weedle::InterfaceDefinition) -> InterfaceConfig{
        let name = interface.identifier.0.to_string();
        let InterfaceMembers { 
            fields,
            methods,
            enums
        } = InterfaceMembers::from_interface(interface);
        
        let interface_struct = InterfaceStruct {
            name,
            fields
        };
        let interface_impl = InterfaceImpl {
                methods: methods
        };
        Self {
            interface_struct,
            interface_impl,
            interface_enums: enums
        }
    }
}

#[derive(Debug)]
struct InterfaceMembers {
    methods: Vec<InterfaceMethod>,
    fields: Vec<InterfaceField>,
    enums: Vec<UnionTypeEnum>
}

impl InterfaceMembers {
    fn from_interface(interface: weedle::InterfaceDefinition) -> Self {
        let members_vec = interface.members.body;
        let mut interface_methods: Vec<InterfaceMethod> = Vec::new();
        let mut interface_fields: Vec<InterfaceField> = Vec::new();
        let mut interface_enums: Vec<UnionTypeEnum> = Vec::new();

        use weedle::interface::InterfaceMember as IM;
        for method in members_vec {
            match method {
                IM::Operation(operation) => {
                    let method_name: String = operation.identifier.expect("Method has no name?").0.to_owned();
                    let mut method_params = InterfaceMethodParam::from_method(operation.clone());
                    let method_type = IDLTypeAsString::from_return_type(&operation.return_type);
                    interface_methods.push(
                        InterfaceMethod {
                            name: method_name,
                            return_type: method_type.0.clone(),
                            params: method_params.0
                        }
                    );
                    if let Some(union_type_enum) = method_type.1 {
                        interface_enums.push(union_type_enum);
                    }
                    interface_enums.append(&mut method_params.1);
                },
                IM::Attribute(attribute) => {
                    let field_name: String = attribute.identifier.0.to_string();
                    let field_type = IDLTypeAsString::from_wtype(&attribute.type_.type_);
                    println!("{:#?}", field_type);
                    interface_fields.push(
                        InterfaceField {
                            field_name,
                            field_type: field_type.0.clone()
                        }
                    );
                    if let Some(union_type_enum) = field_type.1 {
                        interface_enums.push(union_type_enum);
                    }
                }
                _ => {
                    ()
                    // unimplemented!();
                }
            }
        }

        InterfaceMembers { 
            methods: interface_methods,
            fields: interface_fields,
            enums: interface_enums
        }

    }
}

#[derive(Debug)]
struct InterfaceMethodParam {
    param_name: String,
    param_type: String
}
impl InterfaceMethodParam {
    fn from_method(method: OperationInterfaceMember) -> (Vec<Self>, Vec<UnionTypeEnum>) {
        let arguments_vec = method.args.body.list;
        let mut method_params: Vec<Self> = Vec::new();
        let mut method_param_union_type_enums: Vec<UnionTypeEnum> = Vec::new();

        use weedle::argument::Argument as A;
        for argument in arguments_vec {
            match argument {
                A::Single(single ) => {
                    let param_name = single.identifier.0.to_string();
                    let param_type = IDLTypeAsString::from_wtype(&single.type_.type_);
                    method_params.push(
                        Self {
                            param_name,
                            param_type: param_type.0
                        }
                    );
                    if let Some(union_type_enum) = param_type.1 {
                        method_param_union_type_enums.push(union_type_enum);
                    }
                },
                A::Variadic(variadic) => {
                    unimplemented!();
                }
            }
        }
        (method_params, method_param_union_type_enums)
    }
}

#[derive(Debug, Clone)]
struct UnionTypeEnum {
    name: String,
    options: Vec<String>
}

#[derive(Debug)]
struct UnionType {}
impl UnionType {
    fn from_union_types(w_union_types: Vec<UnionMemberType>) -> (String, Option<UnionTypeEnum>) {
        fn get_union_types(union_types: Vec<UnionMemberType>, union_type_names: &mut Vec<String>) {
            for ut in union_types {
                match ut {
                    UnionMemberType::Single(single) => {
                        let union_type = IDLTypeAsString::from_non_any_type(&single.type_);
                        union_type_names.push(union_type);
                    },
                    UnionMemberType::Union(union) => {
                        let union_types_vec = union.type_.body.list;
                        get_union_types(union_types_vec, union_type_names);
                    }
                }
            }
        }
        let mut union_types: Vec<String> = Vec::new();
        get_union_types(w_union_types, &mut union_types);
        let union_type_name = union_types.join("Or");
        let union_type_enum = UnionTypeEnum {
            name: union_type_name.clone(),
            options: union_types
        };
        // println!("{:#?}", union_type_enum);
        (union_type_name, Some(union_type_enum))
    }
}

type TypeAsStringAndMaybeUnionEnum = (String, Option<UnionTypeEnum>);
struct IDLTypeAsString {}
impl IDLTypeAsString {
    fn from_return_type(return_type: &RType) -> TypeAsStringAndMaybeUnionEnum {
        match return_type {
            RType::Type(return_type) => {
                Self::from_wtype(&return_type)
            },
            RType::Undefined(_) => {
                ("".to_owned(), None)
            }
        }
    }

    fn from_wtype(weedle_type: &WType) -> TypeAsStringAndMaybeUnionEnum {
        match weedle_type {
            WType::Single(single) => {
                (Self::from_single_type(single), None)
            },
            WType::Union(union) => {
                let types = union.type_.body.list.clone();
                let result = UnionType::from_union_types(types);
                // println!("{:#?}", result);
                result
            }
        }
    }

    fn from_single_type(single_type: &SType) -> String {
        match single_type {
            SType::Any(any) => {
                todo!();
            },
            SType::NonAny(non_any) => {
                Self::from_non_any_type(non_any)
            }
        }
    }

    fn from_non_any_type(non_any_type: &NAType) -> String {
        use weedle::types::NonAnyType;
        match non_any_type {
            NonAnyType::Promise(v) => {
                unimplemented!();
            },
            NonAnyType::Integer(v) => {
                unimplemented!();
            },
            NonAnyType::FloatingPoint(v) => {
                unimplemented!();
            },
            NonAnyType::Boolean(_) => {
                "bool"
            },
            NonAnyType::Byte(v) => {
                unimplemented!();
            },
            NonAnyType::Octet(v) => {
                unimplemented!();
            },
            NonAnyType::ByteString(v) => {
                unimplemented!();
            },
            NonAnyType::DOMString(v) => {
                "DOMString"
            },
            NonAnyType::USVString(v) => {
                unimplemented!();
            },
            NonAnyType::Sequence(v) => {
                unimplemented!();
            },
            NonAnyType::Object(v) => {
                unimplemented!();
            },
            NonAnyType::Symbol(v) => {
                unimplemented!();
            },
            NonAnyType::Error(v) => {
                unimplemented!();
            },
            NonAnyType::ArrayBuffer(v) => {
                unimplemented!();
            },
            NonAnyType::DataView(v) => {
                unimplemented!();
            },
            NonAnyType::Int8Array(v) => {
                unimplemented!();
            },
            NonAnyType::Int16Array(v) => {
                unimplemented!();
            },
            NonAnyType::Int32Array(v) => {
                unimplemented!();
            },
            NonAnyType::Uint8Array(v) => {
                unimplemented!();
            },
            NonAnyType::Uint16Array(v) => {
                unimplemented!();
            },
            NonAnyType::Uint32Array(v) => {
                unimplemented!();
            },
            NonAnyType::Uint8ClampedArray(v) => {
                unimplemented!();
            },
            NonAnyType::Float32Array(v) => {
                unimplemented!();
            },
            NonAnyType::Float64Array(v) => {
                unimplemented!();
            },
            NonAnyType::ArrayBufferView(v) => {
                unimplemented!();
            },
            NonAnyType::BufferSource(v) => {
                unimplemented!();
            },
            NonAnyType::FrozenArrayType(v) => {
                unimplemented!();
            },
            NonAnyType::RecordType(v) => {
                unimplemented!();
            },
            NonAnyType::Identifier(v) => {
                v.type_.0
            },
        }.into()
    }
}