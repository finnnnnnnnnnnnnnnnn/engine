

// fn main() {
//     let widl = std::fs::read_to_string("test.widl").unwrap();
//     let parsed = weedle::parse(&widl).unwrap();
//     let intermidate = IntermediateStructure::new(parsed);
// }

// struct IntermediateStructure {
//     callback_interfaces: Vec<RustCallbackInterface>,
//     callbacks: Vec<RustCallback>,
//     interfaces: Vec<RustInterface>,
//     interface_mixins: Vec<RustInterfaceMixin>,
//     namespaces: Vec<RustNamespace>,
//     dictionaries: Vec<RustDictionary>
//     // Namespace(NamespaceDefinition<'a>),
//     // Dictionary(DictionaryDefinition<'a>),
//     // PartialInterface(PartialInterfaceDefinition<'a>),
//     // PartialInterfaceMixin(PartialInterfaceMixinDefinition<'a>),
//     // PartialDictionary(PartialDictionaryDefinition<'a>),
//     // PartialNamespace(PartialNamespaceDefinition<'a>),
//     // Enum(EnumDefinition<'a>),
//     // Typedef(TypedefDefinition<'a>),
//     // IncludesStatement(IncludesStatementDefinition<'a>),
//     // Implements(ImplementsDefinition<'a>),
// }

// impl IntermediateStructure {
//     fn new(definitions: Vec<Definition>) -> Self {
//         todo!()
//     }
// }

// struct RustCallback {}
// struct RustCallbackInterface {}
// struct RustInterface {}
// struct RustInterfaceMixin {}
// struct RustNamespace {}
// struct RustDictionary {}
// struct RustPartialInterface {}
// struct RustPartialInterfaceMixin {}
// struct RustPartialDictionary {}
// struct RustPartialNamespace {}
// struct RustEnum {}
// struct RustTypedef {}
// struct RustIncludesStatement {}
// struct RustImplements {}

mod olkd;
use std::{f32::INFINITY, panic::PanicHookInfo};

use codegen as C;
use weedle::{argument::Argument, interface::InterfaceMember, literal::{ConstValue, FloatLit, IntegerLit}, types::{AttributedNonAnyType, AttributedType, ConstType, MayBeNull, NonAnyType, ReturnType, SingleType, Type, UnionMemberType, UnionType}, CallbackInterfaceDefinition, Definition, DictionaryDefinition, InterfaceDefinition};

#[derive(Debug)]
struct RustItems {
    structs: Vec<C::Struct>,
    impls: Vec<C::Impl>,
    enums: Vec<C::Enum>,
}

impl RustItems {
    fn new() -> Self {
        Self {
            structs: Vec::new(),
            impls: Vec::new(),
            enums: Vec::new()
        }
    }

    fn add_from_defintions(&mut self, definitions: Vec<Definition>) {
        use weedle::Definition::*;
        for def in definitions {
            match def {
                CallbackInterface(callback_interface) => { self.add_from_callback_interface(callback_interface);},
                Interface(interface) => { self.add_from_interface(interface); },
                Dictionary(dictionary) => {self.add_from_dictionary(dictionary); },
                _ => { panic!("unimplemented defintion"); }
            }
        }
    }

    fn add_from_callback_interface(&mut self, callback_interface: CallbackInterfaceDefinition) {
        // unimplemented!();
    }

    fn add_from_interface(&mut self, interface: InterfaceDefinition) {
        let name = interface.identifier.0;
        let mut interface_struct = C::Struct::new(name);
        let mut interface_impl = C::Impl::new(name);
        // let mut interface_types: Vec<C::Enum>;
        for member in interface.members.body {
            match member {
                InterfaceMember::Const(const_field) => {
                    let name = const_field.identifier.0;
                    let ty = Self::handle_const_type(const_field.const_type);
                    let value = Self::handle_const_value(const_field.const_value);
                    interface_impl.associate_const(name, ty, value, "pub");
                },
                InterfaceMember::Attribute(attribute) => {
                    // println!("{:#?}", attribute);
                    let name = attribute.identifier.0;
                    let ty = self.handle_attributed_type(attribute.type_);
                    interface_struct.field(name, ty);
                },
                InterfaceMember::Operation(operation) => {
                    let name = operation.identifier.expect("function has no name").0;
                    let ty = self.handle_return_type(operation.return_type);
                    let function = interface_impl.new_fn(name)
                        .ret(ty);
                    for (name, ty) in self.handle_args(operation.args.body.list){
                        function.arg(name, ty);
                    }
                }
                _ => {}
            }
        }
        // println!("{:#?}", interface_struct);
        self.structs.push(interface_struct);
        self.impls.push(interface_impl);

        // //fields can be 
        // let interface_struct = C::Struct::new(name)
        //     .field(name, ty)

        // The rust code for an interface can invole creating many different types and not just adding fields to a struct
        // pub enum InterfaceMember<'a> {
        //     Const(ConstMember<'a>), => constant field, definted in impl
        //     Attribute(AttributeInterfaceMember<'a>), => regular field, can be readonly though, definted in struct
        //     Constructor(ConstructorInterfaceMember<'a>), => 'new' function, defined in impl
        //     Operation(OperationInterfaceMember<'a>), => regular function, definted in impl
        //     Iterable(IterableInterfaceMember<'a>), => I've never actually seen this before, though I assume it would be best handled with a trait, it's going to remain unimplemented until I encounter it though
        //     AsyncIterable(AsyncIterableInterfaceMember<'a>), => same as iteraple
        //     Maplike(MaplikeInterfaceMember<'a>), => same
        //     Setlike(SetlikeInterfaceMember<'a>), => same
        //     Stringifier(StringifierMember<'a>), => same, this is probaly as best handled as a trait thing.
        // }

        //plan for the add_from_interface function:
            // define an empty struct, option<impl>, and vec<enum>
            // pass these into functions which handle the differnet type of interface members and modify
    }


    fn add_from_dictionary(&mut self, dictionary: DictionaryDefinition) {
        // unimplemented!()
    }

    fn handle_args<'a>(&mut self, args: Vec<Argument<'a>>) -> Vec<(&'a str, String)> {
        args
            .iter()
            .map(
                | arg | {
                match arg {
                    Argument::Single(single) => {
                        (single.identifier.0, self.handle_attributed_type(single.type_.clone()))
                    },
                    Argument::Variadic(variadic) => {
                        (variadic.identifier.0, self.handle_type_type(variadic.type_.clone()))
                    }
                }
            }
        )
        .collect()
    }

    fn handle_return_type(&mut self, return_type: ReturnType) -> String{
        match return_type {
            ReturnType::Type(type_type) => { self.handle_type_type(type_type)},
            ReturnType::Undefined(_) => { "None".to_string() }
        }
    }

    fn handle_const_type(const_type: ConstType) -> String {
        match const_type {
            ConstType::Boolean(t) => { Self::handle_may_be_null(t)},
            ConstType::Byte(t) => { Self::handle_may_be_null(t)},
            ConstType::FloatingPoint(t) => { Self::handle_may_be_null(t)},
            ConstType::Integer(t) => { Self::handle_may_be_null(t)},
            ConstType::Identifier(t) => { Self::handle_may_be_null(t)},
            ConstType::Octet(t) => { Self::handle_may_be_null(t)},
        }
    }

    fn handle_const_value(const_value: ConstValue) -> String {
        match const_value {
            ConstValue::Boolean(t) => {t.0.to_string()},
            ConstValue::Float(t) => { 
                match t {
                    FloatLit::Infinity(_) => { "INFINITY".to_string() },
                    FloatLit::NegInfinity(_) => { "NEG_INFINITY".to_string() },
                    FloatLit::Value(value) => {value.0.to_string()},
                    FloatLit::NaN(_) => { "NAN".to_string() },
                }
            },
            ConstValue::Integer(t) => {
                match t {
                    IntegerLit::Dec(dec) => {dec.0.to_string()},
                    IntegerLit::Hex(hex) => {hex.0.to_string()},
                    IntegerLit::Oct(oct) => {oct.0.to_string()},
                }
            },
            ConstValue::Null(_) => { "None".to_string() },
        }
    }


    // fn get_type_name<T>(_: T) -> String {
    //     use std::any::type_name;
    //     type_name::<T>().to_string()
    // }

    fn handle_may_be_null<T>(t: MayBeNull<T>) -> String {
        // refactor to use get_tyoe_name
        // also this can not possible be correct
        use std::any::type_name;
        if t.q_mark.is_some() {
            type_name::<Option<T>>().to_string()
        } else {
            type_name::<T>().to_string()
        }
    }

    fn handle_attributed_non_any_type(atrributed_non_any_type: AttributedNonAnyType) -> String {
        Self::handle_non_any_type(atrributed_non_any_type.type_)
    }

    fn handle_attributed_type(&mut self, atrributed_type: AttributedType) -> String {
        self.handle_type_type(atrributed_type.type_)
    }

    fn handle_type_type(&mut self, type_type: Type) -> String {
        match  type_type {
            Type::Single(single) => { Self::handle_single_type(single) },
            Type::Union(union) => { self.handle_union_type(union.type_)}
        }
    }

    fn handle_single_type(single_type: SingleType) -> String {
        match single_type {
            SingleType::Any(any_type) => { unimplemented!() },
            SingleType::NonAny(non_any_type) => { Self::handle_non_any_type(non_any_type)}
        }
    }


    fn handle_union_type(&mut self, union_type: UnionType) -> String {
        let union_type_items = union_type.body.list;
        let union_type_types: Vec<String> = union_type_items.iter().map(
            |union_type| 
            match union_type {
                UnionMemberType::Single(single_type) => { Self::handle_attributed_non_any_type(single_type.clone()) },
                UnionMemberType::Union(union_type) => { self.handle_union_type(union_type.type_.clone())}
            }
        )
        .collect();
        let name = union_type_types.join("Or");
        let mut union_type_enum = C::Enum::new(&name);
        let _ = union_type_types.iter().map(
            | type_type | {
                union_type_enum.new_variant(type_type)
                    .tuple(&type_type);
            }
        );
        self.enums.push(union_type_enum);
        name
    }

    fn handle_non_any_type(non_any_type: NonAnyType) -> String {
        // this is pretty jank
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



    // fn handle
}


fn main() {
    let mut items = RustItems::new();

    let widl = std::fs::read_to_string("test.widl").unwrap();
    let parsed = weedle::parse(&widl).unwrap();
 
    items.add_from_defintions(parsed);
    println!("{:#?}", items);
}