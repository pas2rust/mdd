use quote::quote;
use syn::Attribute;

use crate::features::to::get_to::{get_to, To};

pub fn generate_to_btreemap(attributes: Vec<Attribute>) -> proc_macro2::TokenStream {
    let To { key } = get_to(attributes);

    quote! {
        use std::collections::BTreeMap;

        pub fn to_btreemap(self) -> BTreeMap<String, Self> {
            let mut map = BTreeMap::new();
            let key = self.#key.to_string();
            map.insert(key, self);
            map
        }
    }
}
