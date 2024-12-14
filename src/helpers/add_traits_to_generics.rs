use syn::{DeriveInput, GenericParam};

pub fn add_traits_to_generics(input: &mut DeriveInput) {
    for param in input.generics.params.iter_mut() {
        if let GenericParam::Type(type_param) = param {
            #[cfg(feature = "builder")]
            type_param
                .bounds
                .push(syn::parse_quote!(::std::default::Default));
            #[cfg(feature = "logging")]
            type_param.bounds.push(syn::parse_quote!(::std::fmt::Debug));
            #[cfg(feature = "json")]
            type_param
                .bounds
                .push(syn::parse_quote!(::serde::Serialize));
            #[cfg(feature = "json")]
            type_param
                .bounds
                .push(syn::parse_quote!(for<'de> ::serde::Deserialize<'de>));
        }
    }
}
