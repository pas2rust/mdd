use quote::quote;
use syn::DeriveInput;

use crate::helpers::get_fields::get_fields;

pub fn generate_to_toml(input: &DeriveInput) -> proc_macro2::TokenStream {
    let toml_entries = get_fields(input).unwrap().iter().map(|field| {
        let field_name = &field.ident;
        quote! {
            table.insert(
                stringify!(#field_name).to_string(),
                toml::Value::try_from(&self.#field_name)
                    .unwrap_or_else(|_| toml::Value::String(format!("{:?}", self.#field_name))),
            );
        }
    });

    quote! {
        pub fn to_toml(&self) -> toml::Value {
            use toml::Value as TomlValue;
            use toml::value::Table;
            let mut table = Table::new();
            #(#toml_entries)*
            toml::Value::Table(table)
        }
    }
}
