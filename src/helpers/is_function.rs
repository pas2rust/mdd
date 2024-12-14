use syn::Type;

pub fn is_function(ty: &Type) -> bool {
    matches!(ty, &Type::BareFn(_))
}
