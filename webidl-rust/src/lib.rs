// use proc_macro::TokenStream;
// use quote::quote;


// #[proc_macro]
// pub fn a_getter(_: TokenStream) -> TokenStream {
//     quote! {
//         fn get_a(&self) -> String;
//     }.into()
// }


// #[proc_macro_derive(TraitWithField)]
// pub fn trait_with_field_derive(input: TokenStream) -> TokenStream { 
//     let ast = syn::parse(input).unwrap();
//     impl_trait_with_field(&ast)
// }

// fn impl_trait_with_field(ast: &syn::DeriveInput) -> TokenStream {
//     let name = &ast.ident;
//     quote! {
//         impl  TraitWithField for #name {
//             fn get_a(&self) -> String {
//                 self.a.clone()
//             }
//         }
//     }.into()
// }