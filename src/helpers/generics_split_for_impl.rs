use syn::{DeriveInput, ImplGenerics, TypeGenerics, WhereClause};

pub fn generics_split_for_impl(
    input: &DeriveInput,
) -> (ImplGenerics<'_>, TypeGenerics<'_>, Option<&WhereClause>) {
    input.generics.split_for_impl()
}
