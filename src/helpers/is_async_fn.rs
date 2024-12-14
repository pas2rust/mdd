use syn::Type;

pub fn is_async_fn(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        type_path
            .path
            .segments
            .iter()
            .any(|segment| segment.ident == "Future")
    } else {
        false
    }
}
