use quote::quote;
use syn::DeriveInput;

use crate::helpers::{get_fields::get_fields, get_struct_name::get_struct_name};

pub fn generate_to_json(input: &DeriveInput) -> proc_macro2::TokenStream {
    let struct_name = get_struct_name(input);
    let json_object = get_fields(input).unwrap().iter().map(|field| {
        let field_name = &field.ident;
        quote! {
            stringify!(#field_name): self.#field_name,
        }
    });

    quote! {
       impl #struct_name {
            pub fn to_json(&self) -> serde_json::Value {
                serde_json::json!({
                    #(#json_object)*
                })
            }
       }
    }
}
