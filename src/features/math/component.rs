use quote::quote;
use syn::DeriveInput;

use crate::helpers::{
    get_named_fields::get_named_fields, get_struct_name::get_struct_name,
    get_type_path::get_type_path, new_ident::new_ident,
};

pub fn generate_math(input: &DeriveInput) -> proc_macro2::TokenStream {
    let struct_name = get_struct_name(input);
    let methods = get_named_fields(input)
        .unwrap()
        .named
        .iter()
        .filter_map(|field| {
            let field_name = field.ident.as_ref().unwrap();
            let field_type = &field.ty;
            let type_path = get_type_path(field_type).unwrap();
            let type_ident = &type_path.path.segments.last().unwrap().ident;
            let method_name_sum = new_ident("sum", field_name);
            let method_name_subtract = new_ident("subtract", field_name);
            let method_name_multiply = new_ident("multiply", field_name);
            let method_name_divide = new_ident("divide", field_name);
            let method_name_inflate = new_ident("inflate", field_name);
            let method_name_discount = new_ident("discount", field_name);

            let basic_numeric_types = [
                "u8", "u16", "u32", "u64", "u128", "usize", "i8", "i16", "i32", "i64", "i128",
                "isize",
            ];
            let basic_methods = quote! {
                pub fn #method_name_inflate(&mut self, percentage: #field_type) {
                    self.#field_name = {
                        let value = self.#field_name as f64;
                        (value + (value * (percentage as f64 / 100.0))) as #field_type
                    };
                }
                pub fn #method_name_discount(&mut self, percentage: #field_type) {
                    self.#field_name = {
                        let value = self.#field_name as f64;
                        (value - (value * (percentage as f64 / 100.0))) as #field_type
                    };
                }
                pub fn #method_name_sum(&mut self, other: #field_type) {
                    self.#field_name += other;
                }
                pub fn #method_name_subtract(&mut self, other: #field_type) {
                    self.#field_name -= other;
                }
                pub fn #method_name_multiply(&mut self, other: #field_type) {
                    self.#field_name *= other;
                }
                pub fn #method_name_divide(&mut self, other: #field_type) {
                    self.#field_name /= other;
                }
            };

            if type_ident == "f32" || type_ident == "f64" {
                let method_name_sqrt = new_ident("sqrt", field_name);
                let method_name_log = new_ident("log", field_name);
                let method_name_round = new_ident("round", field_name);
                let method_name_abs = new_ident("abs", field_name);
                let advanced_methods = quote! {
                    pub fn #method_name_sqrt(&mut self) {
                        self.#field_name = self.#field_name.sqrt();
                    }
                    pub fn #method_name_log(&mut self) {
                        self.#field_name = self.#field_name.ln();
                    }
                    pub fn #method_name_round(&mut self) {
                        self.#field_name = self.#field_name.round();
                    }
                    pub fn #method_name_abs(&mut self) {
                        self.#field_name = self.#field_name.abs();
                    }
                };

                Some(quote! {
                    #basic_methods
                    #advanced_methods
                })
            } else if basic_numeric_types.contains(&type_ident.to_string().as_str()) {
                Some(basic_methods)
            } else {
                None
            }
        });
    quote! {
        impl #struct_name {
            #(#methods)*
        }
    }
}
