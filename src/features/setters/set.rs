use crate::helpers::{
    get_named_fields::get_named_fields, get_struct_name::get_struct_name, new_ident::new_ident,
};
use quote::quote;
use syn::DeriveInput;
pub fn generate_setters(input: &DeriveInput) -> proc_macro2::TokenStream {
    let struct_name = get_struct_name(input);
    let methods = get_named_fields(input).unwrap().named.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        let method_name = new_ident("set", field_name);
        quote! {
            pub fn #method_name<New: Into<#field_type>>(&mut self, new: New) {
                self.#field_name = new.into();
            }
        }
    });

    quote! {
        impl #struct_name {
            #(#methods)*
        }
    }
}
