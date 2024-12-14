use crate::{features::server::repository::disk::get_disk::{get_disk, Disk}, helpers::{Helpers, HelpersTrait}};
use quote::quote;
use syn::Attribute;

pub fn generate_disk_insert_json(
    helpers: Helpers,
    attributes: Vec<Attribute>,
) -> proc_macro2::TokenStream {
    let struct_name = helpers.get_struct_name().unwrap();
    let Disk { unique_fields, path, pretty} = get_disk(struct_name, attributes);

    let compare_unique_fields = unique_fields.iter().map(|ident| {
        quote! {
            for existing_item in &existing_data {
                if let Some(existing_value) = existing_item.get(stringify!(#ident)) {
                    if existing_value == &self.#ident {
                        return Err(format!("[DUPLICATE ENTRY DETECTED FIELD {} ALREADY EXISTS]", stringify!(#ident)));
                    }
                }
            }
        }
    });

    quote! {
        pub async fn disk_insert_json(&self) -> Result<(), String> {
            use serde_json::{Value, to_string_pretty, from_str, to_string};
            use std::path::Path;
            use tokio::{
                fs::{File, OpenOptions, create_dir_all},
                io::{AsyncReadExt, AsyncWriteExt},
            };

            let json_data = self.to_json();
            let path = Path::new(#path).with_extension("json");

            if let Some(parent_dir) = path.parent() {
                create_dir_all(parent_dir).await
                    .map_err(|e| format!("[FAILED TO CREATE DIRECTORY]: {}", e).to_uppercase())?;
            }

            let mut existing_data: Vec<Value> = match File::open(&path).await {
                Ok(mut file) => {
                    let mut content = String::new();
                    file.read_to_string(&mut content).await
                        .map_err(|e| format!("[FAILED TO READ FILE]: {}", e).to_uppercase())?;

                    from_str(&content).map_err(|e| format!("[INVALID JSON]: {}", e).to_uppercase())?
                }
                Err(_) => Vec::new(),
            };

            if existing_data.iter().any(|item| item == &json_data) {
                return Err("[DUPLICATE ENTRY DETECTED]".to_string());
            }

            #(#compare_unique_fields)*;

            existing_data.push(json_data);

            let serialized_content = {
                if #pretty {
                    to_string_pretty(&existing_data)
                        .map_err(|e| format!("[FAILED TO SERIALIZE PRETTY JSON]: {}", e).to_uppercase())
                } else {
                    to_string(&existing_data)
                        .map_err(|e| format!("[FAILED TO SERIALIZE UGLY JSON]: {}", e).to_uppercase())
                }
            }?;

            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(false)
                .open(path)
                .await
                .map_err(|e| format!("[FAILED TO OPEN FILE]: {}", e).to_uppercase())?;

            file.write_all(serialized_content.as_bytes()).await
                .map_err(|e| format!("[FAILED TO WRITE FILE]: {}", e).to_uppercase())?;

            Ok(())
        }
    }
}
