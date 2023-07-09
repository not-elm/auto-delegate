use proc_macro::TokenStream;

use proc_macro2::Span;
use syn::{ItemTrait, LifetimeParam};
use syn::__private::TokenStream2;

use crate::intersperse;
use crate::macro_marker::{expand_macro_maker_name, expand_macro_marker_generics};
use crate::syn::syn_generics::{
    expand_generic_param_without_bound, expand_generics_separate_colon,
    expand_where_bound_without_where_token,
};
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
    let generics = expand_generics_separate_colon(&item.generics);
    let lifetime = generics
        .map(|life_times| quote::quote!(#life_times,));

    let trait_name = expand_trait_name(item);
    let macro_marker_name = expand_macro_maker_name();
    let macro_marker_generics = expand_macro_marker_generics(item.ident.clone());

    let expand_impl = || {
        let impl_generic = proc_macro2::Ident::new("MacroMakerImpl", Span::call_site());
        let trait_functions = trait_functions(item.clone())?;
        let lifetime_bound = expand_lifetimes_bound(item);
        let super_traits = &item.supertraits;
        let where_generics = expand_where_bound_without_where_token(&item.generics);

        Ok(quote::quote! {
         impl<#lifetime #impl_generic> #trait_name for #impl_generic
             where #impl_generic: #macro_marker_name<#macro_marker_generics> + #super_traits,
                   <#impl_generic as #macro_marker_name<#macro_marker_generics>>::DelegateType :  #lifetime_bound,
                   #where_generics
            {
                #(#trait_functions)*
            }
        })
    };

    expand_impl()
}


fn trait_functions(item: ItemTrait) -> syn::Result<Vec<TokenStream2>> {
    let mut trait_fn: Vec<TokenStream2> = Vec::new();

    for fn_token in TraitFnIter::new(item.items).map(|meta| meta.expand_fn()) {
        trait_fn.push(fn_token?);
    }

    Ok(trait_fn)
}


fn expand_trait_name(item: &ItemTrait) -> TokenStream2 {
    let generics: Vec<TokenStream2> = intersperse(
        quote::quote!(,),
        item
            .generics
            .params
            .iter()
            .map(expand_generic_param_without_bound),
    );

    let generics_brackets = if generics.is_empty() {
        None
    } else {
        Some(quote::quote! {
            <#(#generics)*>
        })
    };

    let trait_ident = &item.ident;

    quote::quote! {
        #trait_ident #generics_brackets
    }
}


fn expand_lifetimes_bound(item_trait: &ItemTrait) -> TokenStream2 {
    let trait_name = expand_trait_name(item_trait);

    let lifetime_params: Vec<&LifetimeParam> = item_trait
        .generics
        .lifetimes()
        .collect();

    if lifetime_params.is_empty() {
        quote::quote! {
            #trait_name +  'static
        }
    } else {
        let lifetimes_bound: Vec<TokenStream2> = intersperse(
            quote::quote!(+),
            lifetime_params
                .iter()
                .map(|lifetime| quote::quote!(#lifetime)),
        );


        quote::quote! {
            #trait_name  +  #(#lifetimes_bound)*
        }
    }
}


