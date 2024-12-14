use quote::quote;
use syn::Attribute;

use crate::features::to::get_to::{get_to, To};

pub fn generate_to_btreeset(attributes: Vec<Attribute>) -> proc_macro2::TokenStream {
    let To { key } = get_to(attributes);

    quote! {
        use std::collections::BTreeSet;

        pub fn to_btreeset(self) -> BTreeSet<Self> {
            let mut set = BTreeSet::new();
            set.insert(self);
            set
        }
    }
}
