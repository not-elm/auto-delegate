use proc_macro::{Span, TokenStream};

use proc_macro2::Ident;
use syn::__private::TokenStream2;
use syn::spanned::Spanned;
use syn::{ItemTrait, LifetimeParam};

use crate::span::with_in_tests;
use crate::trait_item::trait_fn_iter::TraitFnIter;

pub fn expand_delegate_trait(_attr: TokenStream, input: TokenStream) -> TokenStream2 {
    match try_expand_delegate_trait(input) {
        Ok(output) => output,
        Err(error) => error.to_compile_error(),
    }
}


fn try_expand_delegate_trait(input: TokenStream) -> syn::Result<TokenStream2> {
    let input_trait = syn::parse::<syn::ItemTrait>(input)?;

    expand_impl_macro(&input_trait)
}


fn expand_impl_macro(item: &ItemTrait) -> syn::Result<TokenStream2> {
    let trait_ident = &item.ident;
    let trait_name = trait_ident
        .clone()
        .to_string();

    let macro_name = format!("impl_delegate_{}", trait_name.to_lowercase());
    let macro_name = Ident::new(&macro_name, proc_macro2::Span::call_site());

    let trait_fn = TraitFnIter::new(item.clone().items).filter_map(|meta| meta.expand_fn().ok());

    let generics = expand_generics(&item);

    Ok(quote::quote! {
        #[macro_export]
        macro_rules! #macro_name{
            () => {

            }
            // ($struct_name: ident, $delegate_field: ident) => {
            //     impl #generics #trait_ident #generics for $struct_name #generics{
            //         #(#trait_fn)*
            //     }
            // };
        }
    })
}


fn trait_mod_path(span: Span) -> Option<TokenStream2> {
    if with_in_tests(&span) {
        None
    } else {
        let bind = span
            .source_file()
            .path()
            .to_str()?
            .to_string()
            .replace(".rs", "");

        let path: Vec<&str> = bind.split('/').collect();

        let expand: TokenStream2 = path.join("::").parse().ok()?;

        Some(expand)
    }
}


fn expand_generics(item: &ItemTrait) -> Option<TokenStream2> {
    let life_times: Vec<&LifetimeParam> = item
        .generics
        .lifetimes()
        .collect();
    if life_times.is_empty() {
        None
    } else {
        Some(quote::quote! {<#(#life_times)*>})
    }
}
