use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Field, LitStr};

use crate::helpers::{get_attr::get_attr, get_attributes::get_attributes};

pub fn validator_pattern(input: &DeriveInput, field: &Field) -> TokenStream {
    let field_name = field.ident.as_ref().expect("field_name must be get");
    let attributes = get_attributes(input);

    let pattern = get_attr::<LitStr>(&attributes, "pattern");

    if let Ok(pattern_exp) = pattern {
        let exp = pattern_exp.value();

        // Regex atualizada
        let re = regex::Regex::new(r#"pattern\(r"([^"]+)",\s*"([^"]+)"\)"#)
            .expect("Erro ao compilar a regex para o padrão");

        let mut regex_pattern = String::new();
        let mut error_message = String::new();

        if let Some(caps) = re.captures(&exp) {
            if let (Some(pattern), Some(error)) = (caps.get(1), caps.get(2)) {
                regex_pattern = pattern.as_str().to_string();
                error_message = error.as_str().trim().to_string();
            }
        }

        quote! {
            let re = regex::Regex::new(#regex_pattern).map_err(|_| #error_message.to_string())?;
            if !re.is_match(&self.#field_name) {
                return Err(#error_message.to_string());
            }
        }
    } else {
        quote! {}
    }
}
