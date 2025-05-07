use std::any::Any;
use std::{clone, fs};

use weedle::argument::Argument;
use weedle::term::Constructor;
use weedle::types::{MayBeNull, NonAnyType, ReturnType, SingleType, Type, UnionMemberType};
use weedle::{common::Identifier};
use weedle::Definition as D;
use weedle::interface::InterfaceMember as IM;
use codegen::{Scope};
use convert_case::{Case, Casing};


fn main() {
    let widl = fs::read_to_string("test.widl").unwrap();
    let parsed = weedle::parse(&widl).unwrap();
    let mut scope = Scope::new();

    for def in parsed {
        match def {
            D::Interface(defintion) => {
                let interface_name = defintion.identifier.0;
                let interface_struct = scope.new_struct(interface_name).derive("Debug");

                if let Some(inheritance) = defintion.inheritance {
                    let inheritance_name = inheritance.identifier.0;
                    interface_struct.field("inheritance", inheritance_name);
                }

                let interface_impl = scope.new_impl(interface_name);
                let interface_members = defintion.members.body;
                for member in interface_members {
                    match member {
                        IM::Operation(operation) => {
                            let operation_name = operation.identifier.unwrap_or_else(|| {panic!("Identifier has no name!")}).0;
                            if operation_name == "new" {
                                panic!("operation is named new!!!!")
                            }
                            let operation_fn = interface_impl.new_fn(&operation_name.to_case(Case::Snake));
                            let return_type = match_return_type(operation.return_type);
                            operation_fn.ret(return_type);
                            let args = operation.args.body.list;
                            for arg in args{
                                match_argument(operation_fn, arg);
                            }
                        },
                        IM::Constructor(constructor) => {
                            let construction_fn = interface_impl.new_fn("new");
                            let args = constructor.args.body.list;
                            for arg in args {
                                match_argument(construction_fn, arg);
                            }
                            
                        }
                        _ => {}
                    }
                }
            },
            _ => {}
        }
    }

    println!("{}", scope.to_string());
}

fn match_argument(function: &mut codegen::Function, arg: Argument<'_>) {
    match arg {
        Argument::Single(single_arg) => {
            let single_arg_type = match_type(single_arg.type_.type_);
            let single_arg_name = single_arg.identifier.0;
            function.arg(single_arg_name, single_arg_type);
        },
        Argument::Variadic(varidic_arg) => {
            todo!()
        }
    }
}

struct Undefined;

impl Into<codegen::Type> for Undefined {
    fn into(self) -> codegen::Type {
        codegen::Type::from("")
    }
}

fn match_return_type(return_type: ReturnType) -> codegen::Type {
    match return_type {
        ReturnType::Undefined(_) => {"".into()},
        ReturnType::Type(returned_type) => {
            match_type(returned_type)
        }
    }

}

fn match_type(returned_type: Type<'_>) -> codegen::Type {
    match returned_type {
        Type::Single(single_type) => {
            match single_type {
                SingleType::Any(single_type_any) => {
                    //do generics!!!!
                    unimplemented!();
                },
                SingleType::NonAny(single_type_non_any) => {
                    match_non_any_type(single_type_non_any).into()
                }
            }
        }
        Type::Union(union_type) => {
            let types = union_type.type_.body.list;
            let mut type_names: Vec<String> = Vec::new();
            get_union_type_names(types, &mut type_names);
            let union_type_name = type_names.join("Or");
            union_type_name.into()
        }
    }
}

fn get_union_type_names(types: Vec<UnionMemberType<'_>>, type_names: &mut Vec<String>) {
    for t in types {
        match t {
            UnionMemberType::Single(non_any) => {
                let single_union_member_type = match_non_any_type(non_any.type_.clone());
                type_names.push(single_union_member_type.to_case(Case::Pascal));
            },
            UnionMemberType::Union(union) => {
                get_union_type_names(union.type_.body.list, type_names);
            }
        }
    }
}

struct DOMString;
impl Into<codegen::Type> for DOMString {
    fn into(self) -> codegen::Type {
        codegen::Type::from("DOMString")
    }
}


fn match_non_any_type(input: NonAnyType) -> String {
    let type_string = match input {
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
    };
    type_string.into()
}

// fn match_may_be_null<T>(input: MayBeNull<T>) -> T {
//     // I have no idea what the heck this means at all
// }