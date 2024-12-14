use quote::quote;
use syn::Attribute;

use crate::features::to::get_to::{get_to, To};

pub fn generate_to_linked_hashmap(attributes: Vec<Attribute>) -> proc_macro2::TokenStream {
    let To { key } = get_to(attributes);

    quote! {
        use linked_hash_map::LinkedHashMap;

        pub fn to_linked_hashmap(self) -> LinkedHashMap<String, Self> {
            let mut map = LinkedHashMap::new();
            let key = self.#key.to_string();
            map.insert(key, self);
            map
        }
    }
}
