use syn::{Type, TypePath};

pub fn get_type_path(ty: &Type) -> Result<&TypePath, &str> {
    match ty {
        Type::Path(type_path) => Ok(type_path),
        Type::Reference(type_reference) => {
            if let Type::Path(type_path) = &*type_reference.elem {
                Ok(type_path)
            } else {
                Err("Expected a Type::Path within the Type::Reference")
            }
        }
        Type::Paren(type_paren) => {
            if let Type::Path(type_path) = &*type_paren.elem {
                Ok(type_path)
            } else {
                Err("Expected a Type::Path within the Type::Paren")
            }
        }
        _ => Err("Unsupported type"),
    }
}
