use syn::LitStr;
use quote::quote;
use crate::helpers::{Helpers, HelpersTrait};

pub fn generate_pgsql_crud(helpers: Helpers) -> proc_macro2::TokenStream {
    let struct_name = helpers.get_struct_name().unwrap();
    let struct_name_snake_case = to_snake_case(&struct_name.to_string());
    let fields = helpers.get_named_fields().unwrap().named.iter();
    let connection_string_attr = Helpers::get_attr::<LitStr>(&helpers.get_attributes(), "connection_string");
    let connection_string = match connection_string_attr {
        Ok(lit_str) => lit_str.value(),
        Err(_) => panic!("Connection string must be provided!"),
    };

    let methods = fields.map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;

        let find_method_name = Helpers::new_ident("pg_find_by_", field_name.clone());
        let delete_method_name = Helpers::new_ident("pg_delete_by_", field_name.clone());

        quote! {
            use crate::app::components::services::env::data::Env;
            use diesel::prelude::*;
            use diesel::r2d2::{self, ConnectionManager};
            use diesel::pg::PgConnection;
            use lazy_static::lazy_static;
            use serde::{Serialize, Deserialize};
            use serde_json::Value;
            use std::LazyLock;
            
            pub static PG_POOL: LazyLock<r2d2::Pool<ConnectionManager<PgConnection>>> = LazyLock::new(|| {
                let manager = ConnectionManager::<PgConnection>::new(#connection_string);
                r2d2::Pool::builder()
                    .build(manager)
                    .expect("Failed to create pool.")
            });

            #[derive(Serialize, Deserialize, Queryable, Insertable)]
            #[table_name = #struct_name_snake_case]
            pub struct #struct_name {
                #(pub #field_name: #field_type),*
            }

            impl #struct_name {
                pub fn #find_method_name(
                    value: &#field_type,
                ) -> Option<Self> {
                    use crate::schema::#struct_name_snake_case::dsl::*;
                    
                    let conn = PG_POOL.get().expect("Failed to get connection from pool");
            
                    let result = #struct_name_snake_case
                        .filter(#field_name.eq(value))
                        .first::<#struct_name>(&conn)
                        .optional()
                        .expect("Error loading data");
            
                    result
                }
            
                pub fn pg_insert(
                    new_value: Self,
                ) -> bool {
                    use crate::schema::#struct_name_snake_case;
            
                    let conn = PG_POOL.get().expect("Failed to get connection from pool");
            
                    let result = diesel::insert_into(#struct_name_snake_case)
                        .values(&new_value)
                        .execute(&conn);
            
                    result.is_ok()
                }
            
                pub fn #delete_method_name(
                    value: &#field_type,
                ) -> bool {
                    use crate::schema::#struct_name_snake_case::dsl::*;
            
                    let conn = PG_POOL.get().expect("Failed to get connection from pool");
            
                    let result = diesel::delete(
                        #struct_name_snake_case.filter(#field_name.eq(value))
                    )
                    .execute(&conn);
            
                    result.is_ok()
                }
            }            
        }
    });

    quote! {
        #(#methods)*
    }
}

// Função para converter o nome da struct para snake_case
fn to_snake_case(name: &str) -> String {
    let mut snake_case_name = String::new();
    let mut prev_char = '_';
    for c in name.chars() {
        if c.is_uppercase() {
            if prev_char != '_' {
                snake_case_name.push('_');
            }
            snake_case_name.push(c.to_lowercase().next().unwrap());
        } else {
            snake_case_name.push(c);
        }
        prev_char = c;
    }
    snake_case_name
}
