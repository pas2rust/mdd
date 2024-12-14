pub fn generate_redis_crud(helpers: Helpers) -> proc_macro2::TokenStream {
    let struct_name = helpers.get_struct_name().unwrap();
    let struct_name_snake_case = to_snake_case(&struct_name.to_string());
    let fields = helpers.get_named_fields().unwrap().named.iter();

    // Definir os métodos com base nos campos da struct
    let methods = fields.map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        let connection_string_attr = helpers.get_attr::<LitStr>(&helpers.get_attributes(), "connection_string");
        let connection_string = match connection_string_attr {
            Some(lit_str) => lit_str.value(),
            None => panic!("Connection string must be provided!"),
        };

        let find_method_name = Helpers::new_ident("redis_find_by_", field_name.clone());
        let delete_method_name = Helpers::new_ident("redis_delete_by_", field_name.clone());

        quote! {
            use redis::Commands;
            use serde::{Deserialize, Serialize};
            use r2d2::{self, Pool};
            use r2d2_redis::RedisConnectionManager;
            use lazy_static::lazy_static;
            
            #[derive(Serialize, Deserialize, Debug)]
            pub struct #struct_name {
                #(pub #field_name: #field_type),*
            }

            lazy_static! {
                /// Pool de conexões Redis estático
                pub static ref REDIS_POOL: Pool<RedisConnectionManager> = {
                    let manager = RedisConnectionManager::new(#connection_string_attr)
                        .expect("Failed to create Redis connection manager");
                    r2d2::Pool::builder()
                        .build(manager)
                        .expect("Failed to create Redis connection pool")
                };
            }

            impl #struct_name {
                /// Busca um registro no Redis por chave
                pub fn #find_method_name(
                    key: &str,
                ) -> redis::RedisResult<Option<Self>> {
                    let conn = REDIS_POOL.get().expect("Failed to get Redis connection");
                    let value: Option<String> = conn.get(key)?;
                    if let Some(json_data) = value {
                        let deserialized: Self = serde_json::from_str(&json_data)
                            .expect("Failed to deserialize Redis value.");
                        Ok(Some(deserialized))
                    } else {
                        Ok(None)
                    }
                }

                /// Insere um novo registro no Redis
                pub fn redis_insert(
                    key: &str,
                    value: &Self,
                ) -> redis::RedisResult<()> {
                    let conn = REDIS_POOL.get().expect("Failed to get Redis connection");
                    let serialized = serde_json::to_string(value)
                        .expect("Failed to serialize value for Redis.");
                    conn.set(key, serialized)?;
                    Ok(())
                }

                /// Remove um registro do Redis por chave
                pub fn #delete_method_name(
                    key: &str,
                ) -> redis::RedisResult<()> {
                    let conn = REDIS_POOL.get().expect("Failed to get Redis connection");
                    conn.del(key)?;
                    Ok(())
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
