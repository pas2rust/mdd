use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::helpers::get_struct_name::get_struct_name;

pub fn generate_to_arc(input: &DeriveInput) -> TokenStream {
    let struct_name = get_struct_name(input);

    quote! {
        impl #struct_name {
            pub fn to_arc(self) -> std::sync::Arc<Self> {
                std::sync::Arc::new(self)
            }
        }
    }
}
