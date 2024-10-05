#![feature(specialization)]

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(ConditionalCopy)]
pub fn conditional_copy_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let generics = input.generics;

    let should_derive = if let Data::Struct(data) = input.data {
        match data.fields {
            Fields::Named(ref fields) => fields.named.iter().all(|f| syn::Type::implements_copy()),
            Fields::Unnamed(ref fields) => {
                fields.unnamed.iter().all(|f| syn::Type::implements_copy())
            }
            Fields::Unit => true,
        }
    } else {
        false
    };

    if should_derive {
        let expanded = quote! {
            impl #generics std::marker::Copy for #name #generics {}
        };
        TokenStream::from(expanded)
    } else {
        TokenStream::new()
    }
}

// Helper function to check if a type implements Copy
trait ImplementsCopy {
    fn implements_copy() -> bool;
}

impl<T> ImplementsCopy for T {
    default fn implements_copy() -> bool {
        false
    }
}

impl<T: Copy> ImplementsCopy for T {
    fn implements_copy() -> bool {
        true
    }
}
