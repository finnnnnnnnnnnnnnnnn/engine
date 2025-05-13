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

    //this is genuinly awful code
    let call_file_path = proc_macro::Span::call_site().file();
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(&call_file_path);
    let content = std::fs::read_to_string(path).unwrap();
    let ast = syn::parse_file(&content).unwrap();
    let structs: Vec<&ItemStruct> = ast.items
        .iter().filter_map(|e| match e {
            Item::Struct(item_struct) => Some(item_struct),
            _ => None,
        })
        .collect();
    
    let parent_struct_name = parent_struct.to_string();
    let struct_names = structs
        .iter().map(|x| {
            x.ident.to_string()
        })
        .collect::<Vec<String>>();

    if !struct_names.contains(&parent_struct_name) {
        panic!("`{}` must one of the structs in `{}`: {:?}", parent_struct_name, call_file_path, struct_names)
    }

    let parent_struct = structs
        .iter().find( |x| {
            x.ident.to_string() == parent_struct_name
        }).unwrap();
    let parent_fields = &parent_struct.fields;

    let mut item_struct = parse_macro_input!(input as ItemStruct);
    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        for field in parent_fields  {
                fields.named.insert(0, field.clone());
        }
        let parser = syn::Field::parse_named;
        let field = parser.parse_str(&["parent:", &parent_struct_name].concat()).unwrap();
        fields.named.insert(0, field);
    }

    
    // println!("{:#?}", structs);

    return quote! {
        #item_struct
    }
    .into();
}
