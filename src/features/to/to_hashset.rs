use quote::quote;

pub fn generate_to_hashset() -> proc_macro2::TokenStream {
    quote! {
        use std::collections::HashSet;

        pub fn to_hashset(self) -> HashSet<Self> {
            let mut set = HashSet::new();
            set.insert(self);
            set
        }
    }
}
