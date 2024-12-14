use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Field, LitStr};

use crate::helpers::{get_attr::get_attr, get_attributes::get_attributes};

pub fn validator_range(input: &DeriveInput, field: &Field) -> TokenStream {
    let field_name = field.ident.as_ref().expect("field_name must be get");
    let attributes = get_attributes(input);

    let range = get_attr::<LitStr>(&attributes, "range");
    if let Ok(range_exp) = range {
        // expected expression format -> "range(min(1, error), max(2, error))"
        let exp = range_exp.value();
        let re = regex::Regex::new(r"min\((\d+),\s*([^)]*)\)|max\((\d+),\s*([^)]*)\)").unwrap();
        let mut min_value = 0;
        let mut min_error = String::new();
        let mut max_value = 0;
        let mut max_error = String::new();

        for cap in re.captures_iter(&exp) {
            if let (Some(value), Some(error)) = (cap.get(1), cap.get(2)) {
                min_value = value.as_str().parse::<i32>().unwrap_or(0);
                min_error = error.as_str().trim().to_string();
            }
            if let (Some(value), Some(error)) = (cap.get(3), cap.get(4)) {
                max_value = value.as_str().parse::<i32>().unwrap_or(0);
                max_error = error.as_str().trim().to_string();
            }
        }

        quote! {
            if self.#field_name > #max_value.into() {
                return Err(#max_error.to_string());
            }
            if self.#field_name < #min_value.into() {
                return Err(#min_error.to_string());
            }
        }
    } else {
        quote! {}
    }
}
