use quote::quote;
use syn::Path;

pub fn get_function_args(syn_path: &Path) -> Option<proc_macro2::TokenStream> {
    if let Some(segment) = syn_path.segments.last() {
        if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
            let args_tokens = args
                .args
                .iter()
                .filter_map(|arg| {
                    if let syn::GenericArgument::Type(ty) = arg {
                        Some(quote! { #ty })
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            return Some(quote! { #(#args_tokens),* });
        }
    }
    None
}
