use syn::Type;

pub fn get_function_signature(ty: &Type) -> Option<(Vec<&Type>, &Type)> {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "fn" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    let mut args_types = Vec::new();
                    for arg in args.args.iter() {
                        if let syn::GenericArgument::Type(ty) = arg {
                            args_types.push(ty);
                        }
                    }
                    if args_types.is_empty() {
                        return None;
                    }
                    let ret_type = args_types.pop().unwrap();
                    return Some((args_types, ret_type));
                }
            }
        }
    }
    None
}
