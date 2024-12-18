use proc_macro2::Span;
use syn::{Attribute, Ident};

#[derive(Debug)]
pub struct To {
    pub key: Ident,
}

pub fn get_to(attributes: &Vec<Attribute>) -> To {
    let mut key: Ident = Ident::new("id", Span::call_site());

    for attr in attributes {
        if attr.path().is_ident("to") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("key") {
                    let value = meta.value()?;
                    let ident: Ident = value.parse()?;
                    key = ident;
                    Ok(())
                } else {
                    Err(meta.error("key attribute must be valid!"))
                }
            })
            .unwrap();
        }
    }

    To { key }
}
