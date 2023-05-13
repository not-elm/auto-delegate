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


fn expand_impl_macro(item: &ItemTrait) -> syn::Result<TokenStream2> {
    let trait_ident = &item.ident;


    let generics = expand_generics(item);
    let lifetime = generics
        .clone()
        .map(|life_times| {
            quote::quote! {
                #life_times,
            }
        });


    let generics_brackets = generics.map(|life_times| {
        quote::quote! {
            <#life_times>
        }
    });
    let trait_ident = quote::quote! {
        #trait_ident #generics_brackets
    };


    let expand_impl = |macro_maker_ident: TokenStream2| {
        let trait_fn: Vec<TokenStream2> = TraitFnIter::new(item.clone().items)
            .map(|meta| meta.expand_fn())
            .try_collect()?;

        let lifetime_bound = expand_generics_bounds(item);

        Ok(quote::quote! {
            impl<#lifetime T, V> #trait_ident for T
                where T: #macro_maker_ident<DelegateType = V>,
                      V: #lifetime_bound
            {
                #(#trait_fn)*
            }
        })
    };


    expand_impl(expand_macro_maker_ident())
}


fn expand_generics(item: &ItemTrait) -> Option<TokenStream2> {
    let life_times: Vec<TokenStream2> = item
        .generics
        .lifetimes()
        .map(|lifetime| quote::quote!(#lifetime))
        .intersperse(quote::quote!(,))
        .collect();

    if life_times.is_empty() {
        None
    } else {
        Some(quote::quote! {#(#life_times)*})
    }
}


fn expand_generics_bounds(
    item_trait: &ItemTrait
) -> TokenStream2 {
    let trait_ident = &item_trait.ident;


    let lifetime_params: Vec<&LifetimeParam> = item_trait
        .generics
        .lifetimes()
        .collect();

    if lifetime_params.is_empty() {
        quote::quote! {
            #trait_ident + 'static
        }
    } else {
        let lifetimes_bound: Vec<TokenStream2> = lifetime_params
            .iter()
            .map(|lifetime| quote::quote!(#lifetime))
            .intersperse(quote::quote!(+))
            .collect();

        let lifetimes = lifetime_params
            .iter()
            .map(|lifetime| quote::quote!(#lifetime))
            .intersperse(quote::quote!(,));

        quote::quote! {
            #trait_ident <#(#lifetimes)*> + #(#lifetimes_bound)*
        }
    }
}