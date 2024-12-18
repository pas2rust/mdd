use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::helpers::get_struct_name::get_struct_name;

pub fn generate_to_tokio_spawn(input: &DeriveInput) -> TokenStream {
    let struct_name = get_struct_name(input);

    quote! {
        impl #struct_name {
            pub fn to_tokio_spawn<F, Fut>(self, f: F) -> tokio::task::JoinHandle<Fut>
            where
                F: FnOnce(Self) -> Fut + Send + 'static,
                Fut: std::future::Future<Output = ()> + Send + 'static,
            {
                tokio::spawn(async move {
                    f(self).await;
                })
            }
        }
    }
}
