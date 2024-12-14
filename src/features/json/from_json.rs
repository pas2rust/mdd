use quote::quote;
use syn::DeriveInput;

use crate::helpers::{get_fields::get_fields, get_struct_name::get_struct_name};

pub fn generate_from_json(input: &DeriveInput) -> proc_macro2::TokenStream {
    let struct_name = get_struct_name(input);
    let from_json_code = {
        let field_deserialization = get_fields(input).unwrap().iter().map(|field| {
            let field_name = &field.ident;
            let _field_type = &field.ty;
            quote! {
                #field_name: match json_object.get(stringify!(#field_name)) {
                    Some(value) => serde_json::from_value(value.clone())
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
       impl #struct_name {
            pub fn from_json(json_value: serde_json::Value) -> Result<Self, String> {
                if let serde_json::Value::Object(json_object) = json_value {
                    Ok(Self {
                        #from_json_code
                    })
                } else {
                    Err(format!(
                        "Invalid JSON format for deserialization: JSON must match the structure '{}'.",
                        stringify!(#struct_name)
                    ))
                }
            }
       }
    }
}
