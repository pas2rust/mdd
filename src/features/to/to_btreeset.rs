use quote::quote;
use syn::DeriveInput;

use crate::helpers::get_struct_name::get_struct_name;

pub fn generate_to_btreeset(input: &DeriveInput) -> proc_macro2::TokenStream {
    let struct_name = get_struct_name(input);

    quote! {
        impl #struct_name {
            pub fn to_btreeset(self) -> std::collections::BTreeSet<Self> {
                let mut set = std::collections::BTreeSet::new();
                set.insert(self);
                set
            }
        }
    }
}
