use crate::{
    features::_server::repository::disk::json::insert::generate_disk_insert_json,
    helpers::{get_named_fields::get_named_fields, get_struct_name::get_struct_name},
};
use quote::quote;
use syn::DeriveInput;

pub fn generate_crud_in_disk(input: &DeriveInput) -> proc_macro2::TokenStream {
    let struct_name = get_struct_name(input);
    let methods = get_named_fields(input).unwrap().named.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;

        quote! {}
    });
    let disk_insert_json = generate_disk_insert_json(input);
    quote! {
        impl #struct_name {
            #disk_insert_json
            #(#methods)*
        }
    }
}
