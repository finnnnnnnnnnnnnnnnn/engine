mod olkd;
use codegen as C;
use weedle::{argument::Argument, interface::InterfaceMember, literal::{ConstValue, FloatLit, IntegerLit}, types::{AttributedNonAnyType, AttributedType, ConstType, MayBeNull, NonAnyType, ReturnType, SingleType, Type, UnionMemberType, UnionType}, CallbackInterfaceDefinition, Definition, DictionaryDefinition, InterfaceDefinition};

fn main() {
    let mut items = CodeContainer::new();

    let widl = std::fs::read_to_string("test.widl").unwrap();
    let parsed = weedle::parse(&widl).unwrap();
 
    items.add_from_defintions(parsed);
    println!("{:#?}", items);
}

#[derive(Debug)]
struct CodeContainer {
    structs: Vec<C::Struct>,
    impls: Vec<C::Impl>,
    enums: Vec<C::Enum>,
}

trait InterfaceAttributes {
    fn associate_const<T: Into<codegen::Type>>(&mut self, name: impl Into<String>, ty: T, value: impl Into<String>, visibility: impl Into<String>) -> &mut Self;
}

impl CodeContainer {
    fn new() -> Self {
        Self {
            structs: Vec::new(),
            impls: Vec::new(),
            enums: Vec::new()
        }
    }

    fn add_from_defintions(&mut self, definitions: Vec<Definition>) {
        // what should I name this function?
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
        let name = callback_interface.identifier.0;
        let mut callback_trait = C::Trait::new(name);
        callback_interface.members.body;
        // pretty sure this is a trait I think maybe not totally sure though
        // I think this also means that the function param parsing code needs to search for these to determine if they are traits because otherwise it assumes they're types
        //          ... ntListener(DOMString type, EventListener? callback, optional (AddEventL ....
        //                                           ^^^^^^^^^ <-- this right here
        // also the ? means it's an optialnal generic too!!!! And I've totally ignored that in parsing types
        // probably good to map the dependency chain so parsing defintions can be done in the correct order
        // also I'd like to more rigoursly handle types, though I feel like static typing makes this difficult
        //      -> I wonder if I could generate the code and then compile as the program is running to check the types 
        //      -> Also this defintily means I need big type maps yayyyyyyy!!!!!!!! -> could I do something with alias'?

        // maybe we do have feintion sepciac types we construct which unwrap into the code container vec

        unimplemented!();
    }

    fn add_from_member<M: InterfaceAttributes>(&mut self, member: T){
        
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
        self.structs.push(interface_struct);
        self.impls.push(interface_impl);
    }


    fn add_from_dictionary(&mut self, dictionary: DictionaryDefinition) {
        // unimplemented!()
    }

    fn handle_args<'a>(&mut self, args: Vec<Argument<'a>>) -> Vec<(&'a str, String)> {
        // this creeps toward a pattenr I think I don't like. would i be better to pass in the function with the arguments? Maybe a Function container is needed?
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
        for type_type in union_type_types {
            union_type_enum.new_variant(type_type.clone()) //you get a clone!
                .tuple(&type_type);
        }
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