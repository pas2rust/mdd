use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod features;
mod helpers;

fn for_extend_token_stream(expanded: &mut TokenStream, tokens: Vec<TokenStream>) {
    for token in tokens {
        expanded.extend::<TokenStream>(token.into());
    }
}

#[cfg(feature = "logging")]
#[proc_macro_derive(Logging, attributes(transporter))]
pub fn logging(input: TokenStream) -> TokenStream {
    use features::logging::{
        print::generate_printers, print_by_field::generate_printers_by_field,
        printers_critical_by_field::generate_printers_critical_by_field,
        printers_err_by_field::generate_printers_err_by_field,
        printers_info_by_field::generate_printers_info_by_field,
        printers_panic_by_field::generate_printers_panic_by_field,
        printers_rust_by_field::generate_printers_rust_by_field,
        printers_success_by_field::generate_printers_success_by_field,
        printers_warning_by_field::generate_printers_warning_by_field,
    };

    let input = parse_macro_input!(input as DeriveInput);

    let mut expanded = TokenStream::new();
    for_extend_token_stream(
        &mut expanded,
        vec![
            generate_printers_by_field(&input).into(),
            generate_printers(&input).into(),
            generate_printers_critical_by_field(&input).into(),
            generate_printers_panic_by_field(&input).into(),
            generate_printers_err_by_field(&input).into(),
            generate_printers_info_by_field(&input).into(),
            generate_printers_rust_by_field(&input).into(),
            generate_printers_warning_by_field(&input).into(),
            generate_printers_success_by_field(&input).into(),
        ],
    );
    expanded.into()
}

#[cfg(feature = "getters")]
#[proc_macro_derive(Getters)]
pub fn getters(input: TokenStream) -> TokenStream {
    use features::getters::{
        get::generate_getters, get_mut::generate_mut_getters, get_ref::generate_ref_getters,
    };

    let input = parse_macro_input!(input as DeriveInput);

    let mut expanded = TokenStream::new();
    for_extend_token_stream(
        &mut expanded,
        vec![
            generate_getters(&input).into(),
            generate_ref_getters(&input).into(),
            generate_mut_getters(&input).into(),
        ],
    );
    expanded.into()
}

#[cfg(feature = "setters")]
#[proc_macro_derive(Setters)]
pub fn setters(input: TokenStream) -> TokenStream {
    use features::setters::set::generate_setters;

    let input = parse_macro_input!(input as DeriveInput);

    let generate_setters = generate_setters(&input);
    generate_setters.into()
}

#[cfg(feature = "builder")]
#[proc_macro_derive(Builder, attributes(pattern, range))]
pub fn builder(input: TokenStream) -> TokenStream {
    use features::builder::component::generate_build;
    let input = parse_macro_input!(input as DeriveInput);

    let generate_build = generate_build(&input);
    generate_build.into()
}

#[cfg(feature = "json")]
#[proc_macro_derive(Json)]
pub fn json(input: TokenStream) -> TokenStream {
    use features::json::{from_json::generate_from_json, to_json::generate_to_json};

    let input = parse_macro_input!(input as DeriveInput);

    let mut expanded = TokenStream::new();
    for_extend_token_stream(
        &mut expanded,
        vec![
            generate_from_json(&input).into(),
            generate_to_json(&input).into(),
        ],
    );
    expanded.into()
}

#[cfg(feature = "math")]
#[proc_macro_derive(
    Math
    //attributes(transporter)
)]
pub fn math(input: TokenStream) -> TokenStream {
    use features::math::component::generate_math;

    let input = parse_macro_input!(input as DeriveInput);

    let generate_math = generate_math(&input);
    generate_math.into()
}

#[cfg(feature = "actix")]
#[proc_macro_derive(
    Actix
    //attributes(transporter)
)]
pub fn actix(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let expanded = quote! {};
    expanded.into()
}

#[cfg(feature = "to")]
#[proc_macro_derive(To, attributes(parser))]
pub fn to(input: TokenStream) -> TokenStream {
    use features::to::{
        to_arc::generate_to_arc, to_arc_mutex::generate_to_arc_mutex, to_box::generate_to_box,
        to_btreemap::generate_to_btreemap, to_cow::generate_to_cow,
        to_hashmap::generate_to_hashmap, to_hashset::generate_to_hashset,
        to_mutex::generate_to_mutex, to_rc::generate_to_rc, to_rc_weak::generate_to_rc_weak,
        to_ref_cell::generate_to_ref_cell, to_tuple::generate_to_tuple, to_vec::generate_to_vec,
        to_vec_tuples::generate_to_vec_tuples,
        tokio::to_arc_tokio_mutex::generate_to_arc_tokio_mutex,
        tokio::to_mutex::generate_to_tokio_mutex,
    };
    let input = parse_macro_input!(input as DeriveInput);
    let attributes = input.attrs.clone();
    let mut expanded = TokenStream::new();
    for_extend_token_stream(
        &mut expanded,
        vec![
            generate_to_arc(&input).into(),
            generate_to_rc(&input).into(),
            generate_to_arc_mutex(&input).into(),
            generate_to_box(&input).into(),
            generate_to_mutex(&input).into(),
            generate_to_hashset(&input).into(),
            generate_to_cow(&input).into(),
            generate_to_tokio_mutex(&input).into(),
            generate_to_arc_tokio_mutex(&input).into(),
            generate_to_vec_tuples(&input, &attributes).into(),
            generate_to_btreemap(&input, &attributes).into(),
            generate_to_rc_weak(&input).into(),
            generate_to_tuple(&input).into(),
            generate_to_ref_cell(&input).into(),
            generate_to_vec(&input).into(),
            generate_to_hashmap(&input, &attributes).into(),
        ],
    );
    expanded.into()
}

#[cfg(feature = "observer")]
#[proc_macro]
pub fn observer(input: TokenStream) -> TokenStream {
    use features::observer::component::observer;
    use syn::ItemEnum;

    let input = parse_macro_input!(input as ItemEnum);
    let expanded = observer(input);
    expanded.into()
}

#[cfg(feature = "hexadecimal")]
#[proc_macro_derive(
    Hexadecimal
    //attributes(transporter)
)]
pub fn hexadecimal(_: TokenStream) -> TokenStream {
    use features::hexadecimal::{from_hex::generate_from_hex, to_hex::generate_to_hex};
    let mut expanded = TokenStream::new();
    for_extend_token_stream(
        &mut expanded,
        vec![generate_to_hex().into(), generate_from_hex().into()],
    );
    expanded.into()
}

#[cfg(feature = "bytes")]
#[proc_macro_derive(
    Bytes
    //attributes(transporter)
)]
pub fn bytes(input: TokenStream) -> TokenStream {
    use features::bytes::{from_bytes::generate_from_bytes, to_bytes::generate_to_bytes};
    let input = parse_macro_input!(input as DeriveInput);
    let mut expanded = TokenStream::new();
    for_extend_token_stream(
        &mut expanded,
        vec![
            generate_to_bytes(&input).into(),
            generate_from_bytes(&input).into(),
        ],
    );
    expanded.into()
}

#[cfg(feature = "virus")]
#[proc_macro]
pub fn virus(_input: TokenStream) -> TokenStream {
    /*let mut file = parse_macro_input!(input as File);
    let mut virus = Virus::new();
    virus.visit_file_mut(&mut file);
    let output = quote! { #file };
    println!("{:#?}", virus);
    println!("{:#?}", output);
    output.into()*/
    quote! {}.into()
}

#[cfg(feature = "repository")]
#[proc_macro_derive(Repository, attributes(disk, db, cache,))]
pub fn repository(input: TokenStream) -> TokenStream {
    use features::_server::repository::disk::component::generate_crud_in_disk;
    let mut expanded = TokenStream::new();
    let input = parse_macro_input!(input as DeriveInput);

    for_extend_token_stream(&mut expanded, vec![generate_crud_in_disk(&input).into()]);
    expanded.into()
}

#[cfg(feature = "service")]
#[proc_macro_derive(Service, attributes())]
pub fn service(input: TokenStream) -> TokenStream {
    quote! {}.into()
}
