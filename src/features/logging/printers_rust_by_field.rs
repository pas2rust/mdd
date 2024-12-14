use quote::quote;
use syn::{DeriveInput, ItemFn};

use crate::helpers::{
    get_attr::get_attr, get_attributes::get_attributes, get_named_fields::get_named_fields,
    get_struct_name::get_struct_name, new_ident::new_ident,
};

pub fn generate_printers_rust_by_field(input: &DeriveInput) -> proc_macro2::TokenStream {
    let struct_name = get_struct_name(input);
    let attributes = get_attributes(input);

    let transporter = match get_attr::<ItemFn>(&attributes, "transporter") {
        Ok(transporter) => transporter.block.stmts,
        Err(_) => Vec::new(),
    };

    let print_field_methods = get_named_fields(input).unwrap().named.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let method_name = new_ident("print_rust", field_name);

        quote! {
            pub async fn #method_name(&self, custom: &str) {
                use colorful::Colorful;
                let message = format!(
                    "({}) @RUST {}.{} => {:#?} {}",
                    chrono::Local::now(),
                    stringify!(#struct_name),
                    stringify!(#field_name),
                    &self.#field_name,
                    custom
                ).rgb(255,165,0);
                #(#transporter)*(message);
            }
        }
    });

    quote! {
        impl #struct_name {
            #(#print_field_methods)*
        }
    }
}
