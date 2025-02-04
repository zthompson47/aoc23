use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn expand_derive_grid(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;
    let mut fields = HashMap::new();

    if let syn::Data::Enum(data) = &input.data {
        for field in &data.variants {
            for attr in &field.attrs {
                if attr.path().is_ident("symbol") {
                    if let syn::Meta::NameValue(attr_meta) = &attr.meta {
                        fields.insert(&field.ident, &attr_meta.value);
                    }
                }
            }
        }
    }

    let (from_char_fields, from_enum_fields) =
        fields
            .iter()
            .fold((quote![], quote![]), |(c, e), (variant, symbol)| {
                (
                    quote![#c #symbol => #name::#variant,],
                    quote![#e #name::#variant => #symbol,],
                )
            });

    quote! {
        impl From<char> for #name {
            fn from(value: char) -> Self {
                match value {
                    #from_char_fields
                    _ => unreachable!(),
                }
            }
        }
        impl From<&#name> for char {
            fn from(value: &#name) -> Self {
                match value {
                    #from_enum_fields
                }
            }
        }
    }
}
