use quote::quote;
use syn::DeriveInput;

use crate::helpers::get_struct_name::get_struct_name;

pub fn generate_to_bytes(input: &DeriveInput) -> proc_macro2::TokenStream {
    let struct_name = get_struct_name(input);
    quote! {
        impl #struct_name {
            pub fn to_bytes(&self) -> Result<Vec<u8>, bincode::Error> {
                bincode::serialize(&self)
            }
        }
    }
}
