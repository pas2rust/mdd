use quote::quote;
use syn::DeriveInput;

use crate::helpers::{
    get_named_fields::get_named_fields, get_struct_name::get_struct_name, new_ident::new_ident,
};

pub fn generate_mut_getters(input: &DeriveInput) -> proc_macro2::TokenStream {
    let struct_name = get_struct_name(input);
    let methods = get_named_fields(input).unwrap().named.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        let method_name = new_ident("get_mut", field_name);
        quote! {
            pub fn #method_name(&mut self) -> &mut #field_type {
                &mut self.#field_name
            }
        }
    });
    quote! {
        impl #struct_name {
            #(#methods)*
        }
    }
}