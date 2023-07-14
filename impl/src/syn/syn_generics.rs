use proc_macro2::TokenStream;
use syn::{GenericParam, Generics};
use syn::__private::TokenStream2;

use crate::intersperse;

pub fn expand_generics_with_brackets(generics: &Generics) -> Option<TokenStream2> {
    expand_generics_separate_colon(generics)
        .map(|generics| {
            quote::quote! {
                 <#generics>
            }
        })
}


pub fn expand_generics_separate_colon(generics: &Generics) -> Option<TokenStream> {
    let generics: Vec<TokenStream2> = intersperse(
        quote::quote!(,),
        generics
            .params
            .iter()
            .map(|p| quote::quote!(#p)),
    );

    if generics.is_empty() {
        None
    } else {
        Some(quote::quote! {
            #(#generics)*
        })
    }
}


pub fn expand_generic_param_without_bound(param: &GenericParam) -> TokenStream2 {
    match param {
        GenericParam::Type(ty) => {
            let param_name = &ty.ident;
            quote::quote!(#param_name)
        }
        _ => quote::quote!(#param)
    }
}


pub fn expand_where_bound(generics: &Generics) -> Option<TokenStream2> {
    generics
        .where_clause
        .as_ref()
        .map(|param| {
            quote::quote!(#param)
        })
}


pub fn expand_where_bound_without_where_token(generics: &Generics) -> Option<TokenStream2> {
    generics
        .where_clause
        .as_ref()
        .map(|where_clause| {
            let params = &where_clause.predicates;
            quote::quote!(#params)
        })
}