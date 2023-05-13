use proc_macro2::{Ident, Span};
use syn::{Type, TypePath, TypeReference, TypeTuple};
use syn::__private::TokenStream2;

pub fn expand_syn_type(ty: &Type) -> Option<TokenStream2> {
    match ty {
        Type::Path(path) => Some(expand_path(path)),
        Type::Tuple(tuple) => expand_tuple_ty(tuple),
        Type::Reference(reference) => Some(expand_ref(reference)),
        _ => None,
    }
}


pub fn syn_type_error(ty: &Type) -> syn::Error {
    let str = format!("Not Supported Type = ({})", quote::quote! {#ty}.to_string());

    syn::Error::new(Span::call_site(), str)
}


fn expand_path(path: &TypePath) -> TokenStream2 {
    let ty = expand_syn_type_path(path);
    quote::quote! {#ty}
}


fn expand_ref(reference: &TypeReference) -> TokenStream2 {
    let elem = &reference.elem;
    let life_time = &reference.lifetime;
    if reference.mutability.is_some() {
        quote::quote! {&#life_time mut #elem}
    } else {
        quote::quote! {&#life_time #elem}
    }
}


fn expand_tuple_ty(tuple: &TypeTuple) -> Option<TokenStream2> {
    let first = tuple.elems.first()?;
    let last = tuple.elems.last()?;
    Some(quote::quote! {(#first, #last)})
}


fn expand_syn_type_path(path: &TypePath) -> &Ident {
    path.path.get_ident().unwrap()
}
