// // https://users.rust-lang.org/t/solved-derive-and-proc-macro-add-field-to-an-existing-struct/52307/3


// use proc_macro::TokenStream;
// use quote::quote;
// use syn::parse::{Parse, ParseStream, Parser, Result};
// use syn::{parse, parse_macro_input, punctuated::Punctuated, Ident, ItemStruct, LitStr, Token};

// // #[proc_macro_attribute]
// // pub fn add_field(args: TokenStream, input: TokenStream) -> TokenStream {
// //     let mut item_struct = parse_macro_input!(input as ItemStruct);
// //     let _ = parse_macro_input!(args as parse::Nothing);

// //     if let syn::Fields::Named(ref mut fields) = item_struct.fields {
// //         fields.named.push(
// //             syn::Field::parse_named
// //                 .parse2(quote! { pub a: String })
// //                 .unwrap(),
// //         );
// //     }

// //     return quote! {
// //         #item_struct
// //     }
// //     .into();
// // }

// #[derive(Debug)]
// struct Args {
//     pub vars: Vec<ItemStruct>,
// }

// impl Parse for Args {
//     fn parse(input: ParseStream) -> Result<Self> {
//         let vars = Punctuated::<syn::ItemStruct, Token![,]>::parse_terminated(input)?;
//         Ok(Args {
//             vars: vars.into_iter().collect::<Vec<ItemStruct>>(),
//         })
//     }
// }

// #[proc_macro_attribute]
// pub fn add_field(args: TokenStream, input: TokenStream) -> TokenStream {
//     let mut item_struct = parse_macro_input!(input as ItemStruct);
//     let args = parse_macro_input!(args as Args);
//     println!("{:#?}", args);

//     let extra_fields = args
//         .vars
//         .iter()
//         .flat_map(|arg| {
//             arg.fields.iter()
//         })
//         .collect::<Vec<_>>();

//     if let syn::Fields::Named(ref mut fields) = item_struct.fields {
//         for field in extra_fields {
//             // I kinda don't think I need htis clone but I'm not really sure what to do
//             fields.named.push(field.clone());
//         }
//     }

//     return quote! {
//         #item_struct
//     }
//     .into();
// }



// // #[proc_macro_attribute]
// // pub fn parent(args: TokenStream, input: TokenStream) -> TokenStream {
// //     let mut item_struct = parse_macro_input!(input as ItemStruct);
// //     let args = parse_macro_input!(args as Args);

// //     let extra_fields = args
// //         .vars
// //         .iter()
// //         .flat_map(|arg| {
// //             let ident = Ident::new(&arg.value(), arg.span());
// //             syn::Field::parse_named.parse2(quote! {
// //                 pub #ident: String
// //             })
// //         })
// //         .collect::<Vec<_>>();

// //     if let syn::Fields::Named(ref mut fields) = item_struct.fields {
// //         for field in extra_fields {
// //             fields.named.push(field);
// //         }
// //     }

// //     return quote! {
// //         #item_struct
// //     }
// //     .into();
// // }




use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn tlborm_fn_macro(_: TokenStream) -> TokenStream {
    quote! {
        fn test(&self, dummy: String);
    }.into()
    // "fn test(dummy: String);".into()
}
