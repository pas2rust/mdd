use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::helpers::get_struct_name::get_struct_name;

pub fn generate_to_cow(input: &DeriveInput) -> TokenStream {
    let struct_name = get_struct_name(input);

    quote! {
        impl #struct_name {
            pub fn to_cow_owned(self) -> std::borrow::Cow<'static, Self> {
                std::borrow::Cow::Owned(self)
            }
            pub fn to_cow_borrowed<'a>(&'a self) -> std::borrow::Cow<'a, Self> {
                std::borrow::Cow::Borrowed(self)
            }
        }
    }
}
