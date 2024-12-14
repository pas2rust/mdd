use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::helpers::get_struct_name::get_struct_name;

pub fn generate_to_arc_mutex(input: &DeriveInput) -> TokenStream {
    let struct_name = get_struct_name(input);

    quote! {
        impl #struct_name {
            pub fn to_arc_mutex(self) -> std::sync::Arc<std::sync::Mutex<Self>> {
                std::sync::Arc::new(std::sync::Mutex::new(self))
            }
        }
    }
}
