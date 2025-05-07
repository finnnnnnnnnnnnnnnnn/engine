use std::fs;
use codegen::Scope;
use weedle::{interface::OperationInterfaceMember, Definition, types::Type as WType, types::SingleType as SType, types::NonAnyType as NAType};

fn main() {
    let widl = fs::read_to_string("test.widl").unwrap();
    let parsed = weedle::parse(&widl).unwrap();
    let mut scope = Scope::new();
}

struct InterfaceMembers {

}

struct InterfaceMethod {
    name: String,
    return_type: String,
    params: Vec<InterfaceMethodParam>
}

struct InterfaceMethodParam {}
struct InterfaceStruct {}
struct InterfaceImpl {}
struct InterfaceEnum {}
struct InterfaceConfig {
    interface_struct: InterfaceStruct,
    interface_impl: InterfaceImpl,
    interface_enums: Vec<InterfaceEnum>
}


impl InterfaceConfig {
    fn new(interface: weedle::InterfaceDefinition) {
        let name = interface.identifier.0;
        let methods = Self::extract_methods(interface);
        
    }

    fn extract_methods(interface: weedle::InterfaceDefinition) {
        let methods_vec = interface.members.body;
        let interface_methods: Vec<InterfaceMethod> = Vec::new();

        use weedle::interface::InterfaceMember as IM;
        for method in methods_vec {
            match method {
                IM::Operation(operation) => {
                    todo!();
                    // interface_methods.push();
                },
                _ => {
                    unimplemented!();
                }
            }
        }
    }

    fn extract_method_params(method: OperationInterfaceMember) -> Vec<InterfaceMethodParam> {
        let arguments_vec = method.args.body.list;
        let method_params: Vec<InterfaceMethodParam> = Vec::new();

        use weedle::argument::Argument as A;
        for argument in arguments_vec {
            match argument {
                A::Single(single ) => {
                    let param_name = single.identifier.0;
                    let param_type = Self::extract_type(single.type_.type_);
                },
                A::Variadic(variadic) => {
                    unimplemented!();
                }
            }
        }
        method_params
    }

    fn extract_type(weedle_type: WType) -> String {
        match weedle_type {
            WType::Single(single) => {
                todo!();
            },
            WType::Union(union) => {
                todo!();
            }
        }
    }

    fn parse_single_type(single_type: SType) -> String {
        match single_type {
            SType::Any(any) => {
                todo!();
            },
            SType::NonAny(non_any) => {
                todo!();
            }
        }
    }

    fn parse_non_any_type(non_any_type: NAType) -> String {
        
    }
}