use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use syn::Attribute;

pub fn find_by_attribute(attrs: &Vec<Attribute>) -> Option<Attribute> {
    attrs
        .iter()
        .find(|attr| {
            attr.meta
                .path()
                .is_ident("to")
        })
        .cloned()
}


pub fn trait_names(by_attr: &Attribute) -> Option<Vec<Ident>> {
    let mut tokens = TokenStream::new();
    by_attr.to_tokens(&mut tokens);

    let mut trait_names: Vec<Ident> = Vec::new();
    by_attr
        .parse_nested_meta(|meta| {
            meta.path
                .segments
                .into_iter()
                .for_each(|s| trait_names.push(s.ident));
            Ok(())
        })
        .ok()?;
    if trait_names.is_empty() {
        None
    } else {
        Some(trait_names)
    }
}
