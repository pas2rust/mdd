use quote::quote;
use syn::Attribute;

use crate::features::to::get_to::{get_to, To};

pub fn generate_to_vec_tuples(attributes: Vec<Attribute>) -> proc_macro2::TokenStream {
    let To { key } = get_to(attributes);

    quote! {
        pub fn to_vec_of_tuples(self) -> Vec<(String, Self)> {
            let key = self.#key.to_string();
            vec![(key, self)]
        }
    }
}
