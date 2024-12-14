use quote::quote;
use syn::DeriveInput;

use crate::helpers::get_struct_name::get_struct_name;

pub fn generate_to_hashset(input: &DeriveInput) -> proc_macro2::TokenStream {
    let struct_name = get_struct_name(input);

    quote! {
        impl #struct_name {
            pub fn to_hashset(self) ->  std::collections::HashSet<Self> {
                let mut set =  std::collections::HashSet::new();
                set.insert(self);
                set
            }
        }
    }
}
