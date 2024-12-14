use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemEnum;

pub fn observer(input: ItemEnum) -> TokenStream {
    let enum_name = &input.ident;
    let variants = &input.variants;

    quote! {
        use async_trait::async_trait;
        use std::{any::Any, collections::HashMap};

        pub type Unknow = Box<dyn Any + Send>;
        pub type Way = HashMap<String, Unknow>;
        pub type Believers = HashMap<String, BoxBeliever>;
        pub type BoxBeliever = Box<dyn Believer + Send>;

        pub enum #enum_name {
            #variants
        }

        #[async_trait]
        pub trait Believer: Send + Sync {
            async fn destiny(&self, event: &mut #enum_name) -> Unknow;
        }

        #[derive(Debug)]
        pub struct Destiny {
            key: Option<String>,
            way: Way,
        }

        impl Destiny {
            pub fn new() -> Self {
                Self {
                    way: HashMap::new(),
                    key: None,
                }
            }
            pub fn way_mut(&mut self) -> &mut HashMap<String, Unknow> {
                &mut self.way
            }

            pub fn set_key(&mut self, new: Option<String>) {
                self.key = new;
            }

            pub fn get_believers_destiny_way_by_filter<T: 'static>(&mut self) -> HashMap<String, T> {
                self.way
                    .drain()
                    .filter_map(|(key, result)| {
                        result.downcast::<T>().ok().map(|val| (key, *val))
                    })
                    .collect()
            }

            pub fn get_believer_destiny_way_by_key<T: 'static>(&mut self) -> Option<T> {
                if let Some(key) = self.key.take() {
                    return self
                        .way
                        .remove(&key)
                        .and_then(|result| result
                            .downcast::<T>()
                            .ok()
                            .map(|boxed| *boxed));
                }
                None
            }
        }

        #[async_trait]
        pub trait Observed {
            fn watch(&mut self, id: String, believer: BoxBeliever);
            fn kill(&mut self, id: String);
            async fn listen(&self, event: &mut #enum_name) -> Destiny;
            fn genocide(&mut self);
            async fn miracle(&mut self, event: &mut #enum_name);
        }

        pub struct God {
            believers: Believers,
        }

        impl God {
            pub fn new() -> Self {
                Self {
                    believers: HashMap::new(),
                }
            }
        }

        #[async_trait]
        impl Observed for God {
            fn watch(&mut self, id: String, believer: BoxBeliever) {
                self.believers.insert(id, believer);
            }

            fn kill(&mut self, id: String) {
                self.believers.remove(&id);
            }

            fn genocide(&mut self) {
                self.believers.clear();
            }

            async fn listen(&self, event: &mut #enum_name) -> Destiny {
                let mut destiny = Destiny::new();
                for (key, believer) in &self.believers {
                    let result = believer.destiny(event).await;
                    destiny.way_mut().insert(key.clone(), result);
                }
                destiny
            }

            async fn miracle(&mut self, event: &mut #enum_name) {
                self.listen(event).await;
                self.genocide();
            }
        }
    }
}
