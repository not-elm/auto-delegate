use proc_macro::TokenStream;

use syn::{ItemTrait, LifetimeParam};
use syn::__private::TokenStream2;

use crate::macro_marker::expand_macro_maker_ident;
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


pub fn expand_dyn_delegate_trait(_attr: TokenStream, input: TokenStream) -> TokenStream2 {
    match try_expand_dyn_delegate_trait(input) {
        Ok(output) => output,
        Err(error) => error.to_compile_error(),
    }
}


fn try_expand_dyn_delegate_trait(input: TokenStream) -> syn::Result<TokenStream2> {
    let input_trait = syn::parse::<syn::ItemTrait>(input)?;

    expand_dyn_impl_macro(&input_trait)
}

fn expand_impl_macro(item: &ItemTrait) -> syn::Result<TokenStream2> {
    let trait_ident = &item.ident;


    let generics = expand_generics(item);
    let life_time = generics.clone().map(|l| {
        quote::quote! {
            #l,
        }
    });


    let g = generics.map(|life_times| {
        quote::quote! {
            <#life_times>
        }
    });
    let trait_ident = quote::quote! {
        #trait_ident #g
    };

    let expand_impl = |macro_maker_ident: TokenStream2| {
        let trait_fn =
            TraitFnIter::new(item.clone().items).filter_map(|meta| meta.expand_fn().ok());

        quote::quote! {
            impl<#life_time T, V> #trait_ident for T
                where T: #macro_maker_ident<DelegateType = V>,
                      V: #trait_ident + 'static
            {
                #(#trait_fn)*
            }
        }
    };


    Ok(expand_impl(expand_macro_maker_ident()))
}


fn expand_dyn_impl_macro(item: &ItemTrait) -> syn::Result<TokenStream2> {
    let trait_ident = &item.ident;


    let _generics = expand_generics(item);

    let expand_impl = |macro_maker_ident: TokenStream2| {
        let trait_fn =
            TraitFnIter::new(item.clone().items).filter_map(|meta| meta.expand_fn().ok());

        quote::quote! {
            impl<T> #trait_ident for T
                where T: define::MacroDynMarker<Box<dyn #trait_ident>>
            {
                #(#trait_fn)*
            }
        }
    };


    Ok(expand_impl(expand_macro_maker_ident()))
}


fn expand_generics(item: &ItemTrait) -> Option<TokenStream2> {
    let life_times: Vec<&LifetimeParam> = item
        .generics
        .lifetimes()
        .collect();
    if life_times.is_empty() {
        None
    } else {
        Some(quote::quote! {#(#life_times)*})
    }
}
