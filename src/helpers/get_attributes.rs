use syn::{Attribute, DeriveInput};

pub fn get_attributes(input: &DeriveInput) -> Vec<Attribute> {
    input.attrs.clone()
}
