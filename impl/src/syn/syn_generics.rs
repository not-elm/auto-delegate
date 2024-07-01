use syn::{GenericParam, Generics};
use syn::__private::TokenStream2;

pub fn expand_generic_param_without_bound(param: &GenericParam) -> TokenStream2 {
    match param {
        GenericParam::Type(ty) => {
            let param_name = &ty.ident;
            quote::quote!(#param_name,)
        }
        _ => quote::quote!(#param,)
    }
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