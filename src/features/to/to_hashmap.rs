use quote::quote;
use syn::Attribute;

use crate::features::to::get_to::{get_to, To};

pub fn generate_to_hashmap(attributes: Vec<Attribute>) -> proc_macro2::TokenStream {
    let To { key } = get_to(attributes);

    quote! {
        use std::collections::HashMap;

        pub fn to_hashmap(self) -> HashMap<String, Self> {
            let mut map = HashMap::new();
            let key = self.#key;
            map.insert(key, self);
            map
        }
    }
}
