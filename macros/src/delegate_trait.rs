use proc_macro::TokenStream;

use syn::__private::TokenStream2;

use crate::macro_marker::expand_macro_maker_ident;
use syn::{ItemTrait, LifetimeParam};


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


    let _generics = expand_generics(item);

    let expand_impl = |macro_maker_ident: TokenStream2| {
        let trait_fn =
            TraitFnIter::new(item.clone().items).filter_map(|meta| meta.expand_fn().ok());

        quote::quote! {
            impl<T: #macro_maker_ident <DelegateType = V>, V: #trait_ident> #trait_ident for T{
                #(#trait_fn)*
            }
        }
    };

    let expand_impls = (0..1)
        .map(expand_macro_maker_ident)
        .map(expand_impl);


    Ok(quote::quote! {
        #(#expand_impls)*
    })
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
