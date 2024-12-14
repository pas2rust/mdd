use crate::helpers::{
    get_attr::get_attr, get_attributes::get_attributes, get_struct_name::get_struct_name,
};
use quote::quote;
use syn::{Attribute, DeriveInput, ItemFn};

pub fn generate_printers(input: &DeriveInput) -> proc_macro2::TokenStream {
    let struct_name = get_struct_name(input);
    let attributes = get_attributes(input);

    let transporter = match get_attr::<ItemFn>(&attributes, "transporter") {
        Ok(transporter) => transporter.block.stmts,
        Err(_) => Vec::new(),
    };

    quote! {
        impl #struct_name {
            pub async fn print_all_levels(&self, custom: &str) {
                self.print(custom).await;
                self.print_rust(custom).await;
                self.print_info(custom).await;
                self.print_success(custom).await;
                self.print_warning(custom).await;
                self.print_err(custom).await;
                self.print_critical(custom).await;
                self.print_panic(custom).await;
            }
            pub async fn print(&self, custom: &str) {
                let message = format!("({}) @PRINT => {:#?} {}", chrono::Local::now(), &self, custom);
                #(#transporter)*(message);
            }
            pub async fn print_rust(&self, custom: &str) {
                use colorful::Colorful;
                let message = format!("({}) @RUST => {:#?} {}", chrono::Local::now(), &self, custom).rgb(255,165,0);
                #(#transporter)*(message);
            }
            pub async fn print_info(&self, custom: &str) {
                use colorful::Colorful;
                let message = format!("({}) @INFO => {:#?} {}", chrono::Local::now(), &self, custom).rgb(0,191,255);
                #(#transporter)*(message);
            }
            pub async fn print_success(&self, custom: &str) {
                use colorful::Colorful;
                let message = format!("({}) @SUCCESS => {:#?} {}", chrono::Local::now(), &self, custom).green();
                #(#transporter)*(message);
            }
            pub async fn print_warning(&self, custom: &str) {
                use colorful::Colorful;
                let message = format!("({}) @WARNING => {:#?} {}", chrono::Local::now(), &self, custom).yellow();
                #(#transporter)*(message);
            }
            pub async fn print_err(&self, custom: &str) {
                use colorful::Colorful;
                let message = format!("({}) @ERROR => {:#?} {}", chrono::Local::now(), &self, custom).magenta();
                #(#transporter)*(message);
            }
            pub async fn print_critical(&self, custom: &str) {
                use colorful::Colorful;
                let message = format!("({}) @CRITICAL => {:#?} {}", chrono::Local::now(), &self, custom).red();
                #(#transporter)*(message);
            }
            pub async fn print_panic(&self, custom: &str) {
                use colorful::Colorful;
                let message = format!("({}) @PANIC => {:#?} {}", chrono::Local::now(), &self, custom).rgb(225,32,254);
                #(#transporter)*(message);
            }
        }
    }
}
