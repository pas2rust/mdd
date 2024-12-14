use syn::{parse::Parse, punctuated::Punctuated, Attribute, Ident, LitBool, LitStr, Token};

#[derive(Debug)]
pub struct Disk {
    pub path: String,
    pub pretty: bool,
    pub unique_fields: Vec<Ident>,
    pub vec: bool,
}

pub fn get_disk(struct_name: Ident, attributes: Vec<Attribute>) -> Disk {
    let mut path = struct_name.to_string();
    let mut pretty = false;
    let mut unique_fields: Vec<syn::Ident> = Vec::new();
    let mut vec = false;

    for attr in attributes {
        if attr.path().is_ident("disk") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("path") {
                    let value = meta.value()?;
                    let s: LitStr = value.parse()?;
                    let path_value = s.value();
                    println!("Path: {}", path_value);
                    path = path_value;
                    Ok(())
                } else if meta.path.is_ident("pretty") {
                    let value = meta.value()?;
                    let b: LitBool = value.parse()?;
                    let pretty_value = b.value();
                    println!("Pretty: {}", pretty_value);
                    pretty = pretty_value;
                    Ok(())
                } else if meta.path.is_ident("unique") {
                    let value = meta.value()?;
                    let idents: Punctuated<Ident, Token![,]> =
                        value.parse_terminated(Ident::parse, Token![,])?;
                    for ident in idents {
                        unique_fields.push(ident);
                    }
                    println!("unique_fields: {:#?}", unique_fields);
                    Ok(())
                } else if meta.path.is_ident("vec") {
                    let value = meta.value()?;
                    let b: LitBool = value.parse()?;
                    let vec_value = b.value();
                    println!("vec: {}", vec_value);
                    vec = vec_value;
                    Ok(())
                } else {
                    Err(meta.error("Atributo não suportado"))
                }
            })
            .unwrap();
        }
    }

    Disk {
        path,
        pretty,
        unique_fields,
        vec,
    }
}
