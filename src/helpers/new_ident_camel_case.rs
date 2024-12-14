use proc_macro2::Ident;

pub fn new_ident_camel_case(prefix: &str, field_name: Ident) -> Ident {
    Ident::new(
        format!("{}{}", prefix, field_name).as_str(),
        field_name.span(),
    )
}
