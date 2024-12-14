use quote::quote;
use syn::DeriveInput;

use crate::helpers::{
    get_named_fields::get_named_fields, get_struct_name::get_struct_name, new_ident::new_ident,
};

pub fn generate_find_in_memory(input: &DeriveInput) -> proc_macro2::TokenStream {
    let struct_name = get_struct_name(input);
    let methods = get_named_fields(input).unwrap().named.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;

        let find_method_name = new_ident("im_find_by", field_name);
        let delete_method_name = new_ident("im_delete_by", field_name);

        quote! {
            mod cache {
                use std::sync::RwLock;
                use std::time::{SystemTime, UNIX_EPOCH};

                #[derive(Debug, Clone)]
                pub struct Exp {
                    pub reads: u64,
                    pub inserts: u64,
                    pub time: u64,
                }

                lazy_static::lazy_static! {
                    // Usando um Vec para armazenar as tuplas (struct, Exp)
                    pub static ref VEC: RwLock<Vec<(super::#struct_name, Exp)>> =
                        RwLock::new(Vec::new());
                }
            }

            impl #struct_name {
                // Método de busca no vetor
                pub fn #find_method_name(value: &#field_type) -> Option<(Self, cache::Exp)>
                where
                    Self: Clone,
                {
                    use cache::VEC;
                    use cache::Exp;

                    let mut vec = VEC.write().unwrap();
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs();

                    for (entity, exp) in vec.iter_mut() {
                        if exp.time < now {
                            vec.retain(|(e, _)| e != entity); // Deleta se o tempo expirou
                            continue;
                        }
                        if *value == entity.clone() {
                            exp.reads += 1;
                            return Some((entity, exp));
                        }
                    }
                    None
                }

                // Método para inserir um valor no vetor
                pub fn im_insert<Key>(key: Key, value: Self, expires_in: u64)
                where
                    Self: Clone,
                {
                    use cache::VEC;
                    use cache::Exp;

                    let mut vec = VEC.write().unwrap();
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs();

                    // Adiciona o item no vetor
                    vec.push((value.clone(), Exp {
                        reads: 0,
                        inserts: 1,
                        time: now + expires_in,
                    }));
                }

                // Método de deleção por valor do campo
                pub fn #delete_method_name(value: &#field_type) -> Option<(Self, cache::Exp)>
                where
                    Self: Clone,
                {
                    use cache::VEC;

                    let mut vec = VEC.write().unwrap();

                    if let Some(pos) = vec.iter().position(|(entity, _)| entity == value) {
                        return Some(vec.remove(pos));
                    }

                    None
                }

                // Método para limpar todos os dados
                pub fn im_clear() {
                    use cache::VEC;

                    let mut vec = VEC.write().unwrap();
                    vec.clear();
                }
            }
        }
    });

    quote! {
        #(#methods)*
    }
}
