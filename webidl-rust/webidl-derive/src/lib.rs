#![feature(proc_macro_span)]

use core::panic;
use std::path::PathBuf;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Parser, Result};
use syn::{Fields, FieldsNamed, Item};
use syn::{parse, parse_macro_input, punctuated::Punctuated, Ident, ItemStruct, LitStr, Token};

#[proc_macro_attribute]
pub fn inherit(parent_struct: TokenStream, input: TokenStream) -> TokenStream {
    // I need to get all of the types that are in scope and search for the one from the args
    // this seems like too much work I'm just going to parse the file for all structs for now.

    // let mut path: PathBuf = env!("CARGO_MANIFEST_DIR").into();
    // path.push(proc_macro::Span::call_site().file());
    // println!("{:#?}", path);
    let path: PathBuf = proc_macro::Span::call_site().file().into();
    let content = std::fs::read_to_string(path).expect("here?");

    let ast = syn::parse_file(&content).unwrap();
    let structs: Vec<&ItemStruct> = ast.items
        .iter().filter_map(|e| match e {
            Item::Struct(item_struct) => Some(item_struct),
            _ => None,
        })
        .collect();

    let parent_struct_name = parent_struct.to_string();
    let parent_struct = structs
        .iter().find( |x| {
            x.ident.to_string() == parent_struct_name
        }).unwrap_or_else(|| {
            let struct_names = structs.iter()
            .map(|x| x.ident.to_string())
            .collect::<Vec<String>>()
            .join(",");
            panic!("`{}` must one of the structs in `{}`", parent_struct_name, struct_names);
        });

    let mut item_struct = parse_macro_input!(input as ItemStruct);
    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        let parent_fields = &parent_struct.fields;
        // this seems kinda bad
        for field in parent_fields  {
                fields.named.insert(0, field.clone());
        }
    }

    return quote! {
        #item_struct
    }
    .into();
}


#[proc_macro]
pub fn tlborm_fn_macro(input: TokenStream) -> TokenStream {
    let binding = input.to_string();
    let ast = weedle::parse(&binding).unwrap();
    assert!(&ast.len() == &1);
    let defintion = ast[0].clone();
    match defintion {
        weedle::Definition::Interface(interface) => { gen_interface(interface) },
        _ => { panic!("unimplemented defintion"); }
    }
}

use weedle::{argument::Argument, interface::InterfaceMember, literal::{ConstValue, FloatLit, IntegerLit}, types::{AttributedNonAnyType, AttributedType, ConstType, MayBeNull, NonAnyType, ReturnType, SingleType, Type, UnionMemberType, UnionType}, CallbackInterfaceDefinition, Definition, DictionaryDefinition, InterfaceDefinition};


fn gen_interface(interface: weedle::InterfaceDefinition) -> TokenStream {
    let interface_name = Ident::new(interface.identifier.0, proc_macro2::Span::call_site());
    let interface_trait_name = Ident::new(&[interface.identifier.0, "Trait"].concat(), proc_macro2::Span::call_site());
    let interface_methods = interface.members.body
        .iter()
        .filter_map( | member | {
            match member {
                InterfaceMember::Operation(operation) => {
                    let name = Ident::new(operation.identifier.expect("function has no name").0, proc_macro2::Span::call_site());
                    let ty = Ident::new(&get_return_type(operation.return_type.clone()), proc_macro2::Span::call_site());

                    // for (name, ty) in self.handle_args(operation.args.body.list){
                    //     function.arg(name, ty);
                    // }
                    Some(
                        quote! {
                            fn #name() -> #ty;
                        }
                    )
                },
                _ => { None }
            }
        });

    let out = quote! {
        #[derive(Debug)]
        struct #interface_name {
        }

        trait #interface_trait_name {

        }
        static_assertions::assert_impl_all!(#interface_name: #interface_trait_name); //will this work
    };
    println!("{:#?}", out);
    out.into()
}

fn get_return_type(return_type: ReturnType) -> String{
    match return_type {
        ReturnType::Type(type_type) => { get_type_type(type_type)},
        ReturnType::Undefined(_) => { "None".to_string() }
    }
}

fn get_type_type(type_type: Type) -> String {
    match  type_type {
        Type::Single(single) => { get_single_type(single) },
        Type::Union(union) => { get_union_type(union.type_)}
    }
}


fn get_single_type(single_type: SingleType) -> String {
    match single_type {
        SingleType::Any(any_type) => { unimplemented!() },
        SingleType::NonAny(non_any_type) => { get_non_any_type(non_any_type)}
    }
}


fn get_union_type(union_type: UnionType) -> String {
    let union_type_items = union_type.body.list;
    let union_type_types: Vec<String> = union_type_items.iter().map(
        |union_type| 
        match union_type {
            UnionMemberType::Single(single_type) => { get_attributed_non_any_type(single_type.clone()) },
            UnionMemberType::Union(union_type) => { get_union_type(union_type.type_.clone())}
        }
    )
    .collect();
    let name = union_type_types.join("Or");
    // let mut union_type_enum = C::Enum::new(&name);
    // for type_type in union_type_types {
    //     union_type_enum.new_variant(type_type.clone()) //you get a clone!
    //         .tuple(&type_type);
    // }
    // self.enums.push(union_type_enum);
    name
}

fn get_attributed_non_any_type(atrributed_non_any_type: AttributedNonAnyType) -> String {
    get_non_any_type(atrributed_non_any_type.type_)
}

fn get_attributed_type(atrributed_type: AttributedType) -> String {
    get_type_type(atrributed_type.type_)
}

fn get_non_any_type(non_any_type: NonAnyType) -> String {
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
