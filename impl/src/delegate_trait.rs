use proc_macro::TokenStream;

use proc_macro2::Span;
use syn::{GenericParam, ItemTrait, LifetimeParam, TypeParam, TypeParamBound};
use syn::__private::TokenStream2;
use syn::punctuated::Punctuated;
use syn::token::Plus;

use crate::delegatable::delegatable_ident_with_generics;
use crate::intersperse;
use crate::syn::syn_generics::{
    expand_generic_param_without_bound,
    expand_where_bound_without_where_token,
};
use crate::trait_item::functions::TraitFunctions;

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
    let trait_name = expand_trait_name(item);
    let delegatable = delegatable_ident_with_generics(item.ident.clone());

    let impl_generic = proc_macro2::Ident::new("DelegateImpl", Span::call_site());
    let trait_functions = trait_functions(item.clone())?;

    let mut generics = item.generics.clone();
    generics.params.push(GenericParam::Type(TypeParam::from(impl_generic.clone())));

    let lifetime_bound = expand_lifetimes_bound(item);
    let super_traits = super_traits_bound(&item.supertraits);
    let where_generics = expand_where_bound_without_where_token(&item.generics);

    Ok(quote::quote! {
         impl #generics #trait_name for #impl_generic
             where #impl_generic: #delegatable #super_traits,
                <#impl_generic as #delegatable>::A :  #lifetime_bound,
                <#impl_generic as #delegatable>::B :  #lifetime_bound,
                <#impl_generic as #delegatable>::C :  #lifetime_bound,
                <#impl_generic as #delegatable>::D :  #lifetime_bound,
                <#impl_generic as #delegatable>::D :  #lifetime_bound,
                <#impl_generic as #delegatable>::E :  #lifetime_bound,
                <#impl_generic as #delegatable>::F :  #lifetime_bound,
                <#impl_generic as #delegatable>::G :  #lifetime_bound,
                <#impl_generic as #delegatable>::H :  #lifetime_bound,
                <#impl_generic as #delegatable>::I :  #lifetime_bound,
                <#impl_generic as #delegatable>::J :  #lifetime_bound,
                <#impl_generic as #delegatable>::K :  #lifetime_bound,
                <#impl_generic as #delegatable>::L :  #lifetime_bound,
                #where_generics
            {
                #(#trait_functions)*
            }
    })
}


fn super_traits_bound(super_traits: &Punctuated<TypeParamBound, Plus>) -> Option<TokenStream2> {
    if super_traits.is_empty() {
        None
    } else {
        Some(quote::quote!(+ #super_traits))
    }
}


fn trait_functions(item: ItemTrait) -> syn::Result<Vec<TokenStream2>> {
    let mut trait_fn: Vec<TokenStream2> = Vec::new();
    let delegatable_ident = delegatable_ident_with_generics(item.ident);
    for fn_token in TraitFunctions::new(item.items).map(|mut meta| meta.expand_fn(&delegatable_ident)) {
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
            #trait_name
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


