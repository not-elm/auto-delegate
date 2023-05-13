use proc_macro2::TokenStream;
use syn::{GenericParam, Generics};
use syn::__private::TokenStream2;

pub fn expand_generics_with_brackets(generics: &Generics) -> Option<TokenStream2> {
    expand_generics_separate_colon(generics)
        .map(|generics| {
            quote::quote! {
                 <#generics>
            }
        })
}


pub fn expand_generics_separate_colon(generics: &Generics) -> Option<TokenStream> {
    let generics: Vec<TokenStream2> = generics
        .params
        .iter()
        .map(|p| quote::quote!(#p))
        .intersperse(quote::quote!(,))
        .collect();

    if generics.is_empty() {
        None
    } else {
        Some(quote::quote! {
            #(#generics)*
        })
    }
}


pub fn expand_generics_with_brackets_without_bound(generics: &Generics) -> TokenStream2 {
    let expand: Vec<TokenStream2> = generics
        .params
        .iter()
        .map(expand_generic_param_without_bound)
        .intersperse(quote::quote!(,))
        .collect();

    quote::quote! {
        <#(#expand)*>
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