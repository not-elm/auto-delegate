use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;
use syn::Attribute;

pub fn find_to_attribute(attrs: &Vec<Attribute>) -> Option<Attribute> {
    attrs
        .iter()
        .find(|attr| {
            attr.meta
                .path()
                .is_ident("to")
        })
        .cloned()
}


pub fn syn_error_least_one_give_to_attribute() -> syn::Error {
    syn::Error::new(Span::call_site(), "At least one 'to' attribute must be given.")
}


pub fn syn_error_least_one_trait() -> syn::Error {
    syn::Error::new(Span::call_site(), "At least one trait name must be specified.")
}


pub fn trait_names(to_attr: &Attribute) -> Option<Vec<Ident>> {
    let mut tokens = TokenStream::new();
    to_attr.to_tokens(&mut tokens);

    let mut trait_names: Vec<Ident> = Vec::new();
    to_attr
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
