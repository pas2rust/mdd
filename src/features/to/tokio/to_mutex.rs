use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::helpers::get_struct_name::get_struct_name;

pub fn generate_to_tokio_mutex(input: &DeriveInput) -> TokenStream {
    let struct_name = get_struct_name(input);

    quote! {
        impl #struct_name {
            pub fn to_tokio_mutex(self) -> tokio::sync::Mutex<Self> {
                tokio::sync::Mutex::new(self)
            }
        }
    }
}
