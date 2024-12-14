use syn::{parse::Parse, Attribute};

pub fn get_attr<T: Parse>(attributes: &[Attribute], name: &str) -> Result<T, String> {
    attributes
        .iter()
        .find(|attr| attr.path().is_ident(name))
        .map_or(Err(format!("Attribute {} not found", name)), |attr| {
            attr.parse_args::<T>()
                .map_err(|_| format!("Failed to parse attribute {}", name))
        })
}
