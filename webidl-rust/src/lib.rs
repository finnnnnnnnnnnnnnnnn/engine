#![feature(proc_macro_span)]

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Parser, Result};
use syn::Item;
use syn::{parse, parse_macro_input, punctuated::Punctuated, Ident, ItemStruct, LitStr, Token};

#[proc_macro_attribute]
pub fn add_field(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);
    let _ = parse_macro_input!(args as parse::Nothing);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub a: String })
                .unwrap(),
        );
    }

    return quote! {
        #item_struct
    }
    .into();
}

#[derive(Debug)]
struct Args {
    pub vars: Vec<LitStr>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let vars = Punctuated::<syn::LitStr, Token![,]>::parse_terminated(input)?;
        Ok(Args {
            vars: vars.into_iter().collect::<Vec<LitStr>>(),
        })
    }
}

#[proc_macro_attribute]
pub fn add_field2(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);
    let args = parse_macro_input!(args as Args);

    let extra_fields = args
        .vars
        .iter()
        .flat_map(|arg| {
            let ident = Ident::new(&arg.value(), arg.span());
            syn::Field::parse_named.parse2(quote! {
                pub #ident: String
            })
        })
        .collect::<Vec<_>>();

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        for field in extra_fields {
            fields.named.push(field);
        }
    }

    return quote! {
        #item_struct
    }
    .into();
}



// #[proc_macro_attribute]
// pub fn inherit(args: TokenStream, input: TokenStream) -> TokenStream {
//     let root = env!("CARGO_MANIFEST_DIR");
//     let this_file = file!();
//     println!("{:#?}", this_file);
//     // return quote! {
//     // }
//     // .into();
//     return input;
// }

#[proc_macro_attribute]
pub fn inherit(args: TokenStream, input: TokenStream) -> TokenStream {
    // I need to get all of the types that are in scope and search for the one from the args
    // this seems like too much work I'm just going to parse the file for all structs for now.

    let call_file_path = proc_macro::Span::call_site().file();
    // let ast = syn::parse_file(&call_file_path);
    if let Ok(ast) = syn::parse_file(&call_file_path) {
        let structs: Vec<&ItemStruct> = ast.items
        .iter().filter_map(|e| match e {
            Item::Struct(item_struct) => Some(item_struct),
            _ => None,
        })
        .collect();
        println!("{:#?}", structs);
    }

    input
}
