use quote::quote;
use syn::DeriveInput;

use crate::helpers::get_struct_name::get_struct_name;

pub fn generate_from_bytes(input: &DeriveInput) -> proc_macro2::TokenStream {
    let struct_name = get_struct_name(input);
    quote! {
        impl #struct_name {
            pub fn from_bytes(bin: &[u8]) -> Result<Self, bincode::Error> {
                bincode::deserialize(bin)
            }
        }
    }
}
