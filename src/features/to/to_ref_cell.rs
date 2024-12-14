use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::helpers::get_struct_name::get_struct_name;

pub fn generate_to_ref_cell(input: &DeriveInput) -> TokenStream {
    let struct_name = get_struct_name(input);

    quote! {
        impl #struct_name {
            pub fn to_ref_cell(self) -> std::cell::RefCell<Self> {
                std::cell::RefCell::new(self)
            }
        }
    }
}
