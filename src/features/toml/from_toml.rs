use quote::quote;
use syn::DeriveInput;

use crate::helpers::{get_fields::get_fields, get_struct_name::get_struct_name};

pub fn generate_from_toml(input: &DeriveInput) -> proc_macro2::TokenStream {
    let struct_name = get_struct_name(input);

    let from_toml_code = {
        let field_deserialization = get_fields(input).unwrap().iter().map(|field| {
            let field_name = &field.ident;
            quote! {
                #field_name: match table.get(stringify!(#field_name)) {
                    Some(value) => toml::Value::try_into(value.clone())
                        .unwrap_or_default(),
                    None => Default::default(),
                },
            }
        });

        quote! {
            #(#field_deserialization)*
        }
    };
    quote! {
        pub fn from_toml(toml_value: toml::Value) -> Result<Self, String> {
            if let toml::Value::Table(table) = toml_value {
                Ok(Self {
                    #from_toml_code
                })
            } else {
                Err(format!(
                    "Formato TOML inválido para desserialização: TOML deve corresponder à estrutura '{}'.",
                    stringify!(#struct_name)
                ))
            }
        }
    }
}
