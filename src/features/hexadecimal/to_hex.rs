use quote::quote;

pub fn generate_to_hex() -> proc_macro2::TokenStream {
    quote! {
        pub fn to_hex(&self) -> Result<String, bincode::Error> {
            let bytes = self.to_bytes()?;
            Ok(hex::encode(bytes))
        }
    }
}
