use quote::quote;
use syn::{Attribute, DeriveInput};

use crate::{
    features::to::get_to::{get_to, To},
    helpers::get_struct_name::get_struct_name,
};

pub fn generate_to_vec_tuples(
    input: &DeriveInput,
    attributes: Vec<Attribute>,
) -> proc_macro2::TokenStream {
    let To { key } = get_to(attributes);
    let struct_name = get_struct_name(input);

    quote! {
        impl #struct_name {
            pub fn to_vec_of_tuples(self) -> Vec<(String, Self)> {
                let key = &self.#key.to_string();
                vec![(key, self)]
            }
        }
    }
}
