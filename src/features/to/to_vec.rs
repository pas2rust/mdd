use quote::quote;

pub fn generate_to_vec() -> proc_macro2::TokenStream {
    quote! {
        pub fn to_vec(self) -> Vec<Self> {
           vec![self]
        }
    }
}
