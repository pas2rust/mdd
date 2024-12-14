use crate::helpers::{
    get_named_fields::get_named_fields, get_struct_name::get_struct_name, is_async_fn::is_async_fn,
};

use super::{validator_pattern::validator_pattern, validator_range::validator_range};
use quote::quote;
use syn::DeriveInput;

pub fn generate_build(input: &DeriveInput) -> proc_macro2::TokenStream {
    let struct_name = get_struct_name(input);
    let iter = get_named_fields(input).unwrap().named.iter();
    let methods = iter.clone().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        let is_async_fn = is_async_fn(field_type);

        if is_async_fn {
            quote! {
                pub async fn #field_name<Darth: Into<#field_type>>(mut self, new: Darth) -> Self {
                    self.#field_name = new.into().await;
                    self
                }
            }
        } else {
            quote! {
                pub fn #field_name<Darth: Into<#field_type>>(mut self, new: Darth) -> Self {
                    self.#field_name = new.into();
                    self
                }
            }
        }
    });
    let check_pattern = iter.clone().map(|field| validator_pattern(input, field));
    let check_range = iter.clone().map(|field| validator_range(input, field));
    let async_fields_init = iter.clone().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        let is_async_fn = is_async_fn(field_type);

        if is_async_fn {
            quote! {
                #field_name: instance.#field_name.await
            }
        } else {
            quote! {
                #field_name: instance.#field_name
            }
        }
    });

    let impl_block = quote! {
        impl #struct_name {
            pub fn new() -> Self {
                Self::default()
            }

            pub fn build(self) -> Result<Self, String> {
                #(#check_pattern)*
                #(#check_range)*
                Ok(self)
            }

            pub async fn async_new() -> Self {
                let instance = Self::default();
                Self {
                    #(#async_fields_init),*
                }
            }

            #(#methods)*
        }
    };

    impl_block
}
