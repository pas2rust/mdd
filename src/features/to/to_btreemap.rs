use quote::quote;
use syn::{Attribute, DeriveInput};

use crate::{
    features::to::get_to::{get_to, To},
    helpers::get_struct_name::get_struct_name,
};

pub fn generate_to_btreemap(
    input: &DeriveInput,
    attributes: &Vec<Attribute>,
) -> proc_macro2::TokenStream {
    let To { key } = get_to(attributes);
    let struct_name = get_struct_name(input);

    quote! {
        impl #struct_name {
            pub fn to_btreemap(self) -> std::collections::BTreeMap<String, Self> {
                let mut map = std::collections::BTreeMap::new();
                let key = self.#key.to_string();
                map.insert(key, self);
                map
            }
        }
    }
}
