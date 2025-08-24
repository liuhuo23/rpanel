mod task;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, FieldsNamed, parse_macro_input};

#[proc_macro_derive(Describe)]
pub fn derive_describe(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    let field_names = match data {
        Data::Struct(s) => match s.fields {
            syn::Fields::Named(FieldsNamed { ref named, .. }) => {
                let idents = named.iter().map(|f| &f.ident);
                quote! {concat!{stringify!(#(#idents),*)}}
            }
            _ => panic!("Describe 只能用于具名字段的结构体"),
        },
        _ => panic!("Describe 只能用于结构体"),
    };
    let expanded = quote! {
        impl #ident {
            fn describe() {
                println!("{} fields: {}", stringify!(#ident), #field_names);
            }
        }
    };
    expanded.into()
}

#[proc_macro_attribute]
pub fn queue_task(args: TokenStream, input: TokenStream) -> TokenStream {
    task::task_meta_impl(args, input)
}
