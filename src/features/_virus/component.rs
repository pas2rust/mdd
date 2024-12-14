use std::collections::HashMap;
use rand::{thread_rng, SeedableRng, Rng};
use rand_chacha::ChaCha20Rng;
use syn::{
    visit_mut::{self, VisitMut},
    *,
};
use quote::quote;

#[derive(Debug)]
struct Virus {
    idents: HashMap<String, String>,
    rng: ChaCha20Rng, // Gerador de números pseudoaleatórios com seed
}

impl Virus {
    /// Cria uma nova instância com uma seed
    pub fn new(seed: u64) -> Self {
        Self {
            idents: HashMap::new(),
            rng: ChaCha20Rng::seed_from_u64(seed), // Gerador inicializado com a seed
        }
    }

    /// Gera ou retorna um nome obfuscado a partir de um original
    fn get_or_generate_random_ident(&mut self, original_name: &str) -> Ident {
        if let Some(random_string) = self.idents.get(original_name) {
            Ident::new(random_string, proc_macro2::Span::call_site())
        } else {
            let length = self.rng.gen_range(8..=16); // Nome entre 8 e 16 caracteres
            let random_string: String = (0..length)
                .map(|_| {
                    let char_pool = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_";
                    char_pool[self.rng.gen_range(0..char_pool.len())] as char
                })
                .collect();

            self.idents.insert(original_name.to_string(), random_string.clone());
            Ident::new(&random_string, proc_macro2::Span::call_site())
        }
    }

    /// Renomeia os itens em `use` (ex: `use x::y`)
    fn rename_use_tree(&mut self, tree: &mut UseTree) {
        match tree {
            UseTree::Name(name) => {
                let use_name = name.ident.to_string();
                name.ident = self.get_or_generate_random_ident(&use_name);
            }
            UseTree::Path(path) => {
                let path_name = path.ident.to_string();
                path.ident = self.get_or_generate_random_ident(&path_name);
                self.rename_use_tree(&mut path.tree);
            }
            UseTree::Group(group) => {
                for item in &mut group.items {
                    self.rename_use_tree(item);
                }
            }
            _ => {}
        }
    }
}

impl VisitMut for Virus {
    fn visit_item_impl_mut(&mut self, item_impl: &mut ItemImpl) {
        // Renomear o nome da struct
        if let Type::Path(type_path) = &mut *item_impl.self_ty {
            if let Some(last_segment) = type_path.path.segments.last_mut() {
                let type_name = last_segment.ident.to_string();
                last_segment.ident = self.get_or_generate_random_ident(&type_name);
            }
        }

        // Renomear a trait implementada
        if let Some((_, ref mut path, _)) = &mut item_impl.trait_ {
            if let Some(last_segment) = path.segments.last_mut() {
                let trait_name = last_segment.ident.to_string();
                last_segment.ident = self.get_or_generate_random_ident(&trait_name);
            }
        }

        // Renomear métodos dentro do impl
        for impl_item in &mut item_impl.items {
            if let ImplItem::Fn(method) = impl_item {
                let method_name = method.sig.ident.to_string();
                method.sig.ident = self.get_or_generate_random_ident(&method_name);
                self.visit_block_mut(&mut method.block);
            }
        }

        visit_mut::visit_item_impl_mut(self, item_impl);
    }

    fn visit_expr_path_mut(&mut self, expr_path: &mut ExprPath) {
        if let Some(ident) = expr_path.path.segments.last_mut() {
            let path_name = ident.ident.to_string();
            if let Some(new_name) = self.idents.get(&path_name) {
                ident.ident = Ident::new(new_name, ident.ident.span());
            }
        }
        visit_mut::visit_expr_path_mut(self, expr_path);
    }

    fn visit_item_struct_mut(&mut self, item_struct: &mut ItemStruct) {
        let original_name = item_struct.ident.to_string();
        item_struct.ident = self.get_or_generate_random_ident(&original_name);

        if let Fields::Named(fields_named) = &mut item_struct.fields {
            for field in &mut fields_named.named {
                if let Some(ref ident) = field.ident {
                    let field_name = ident.to_string();
                    field.ident = Some(self.get_or_generate_random_ident(&field_name));
                }
            }
        }

        visit_mut::visit_item_struct_mut(self, item_struct);
    }

    fn visit_item_mod_mut(&mut self, item_mod: &mut ItemMod) {
        let mod_name = item_mod.ident.to_string();
        item_mod.ident = self.get_or_generate_random_ident(&mod_name);
        visit_mut::visit_item_mod_mut(self, item_mod);
    }

    fn visit_item_use_mut(&mut self, item_use: &mut ItemUse) {
        self.rename_use_tree(&mut item_use.tree);
        visit_mut::visit_item_use_mut(self, item_use);
    }
}
