use quote::quote;

pub fn generate_from_hex() -> proc_macro2::TokenStream {
    quote! {
        pub fn from_hex(h: String) -> Result<Self, Box<dyn std::error::Error>> {
            let bytes = hex::decode(h)?;
            Self::from_bytes(&bytes).map_err(|e| e.into())
        }
    }
}
