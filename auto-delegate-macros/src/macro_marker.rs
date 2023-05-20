use proc_macro2::Ident;
use syn::__private::TokenStream2;

use crate::ident::ident_to_lower_string;

pub fn expand_macro_maker_ident(trait_ident: Ident) -> TokenStream2 {
    let name = expand_macro_maker_name();
    let generics = expand_macro_marker_generics(trait_ident);

    quote::quote!(#name<#generics>)
}


pub fn expand_macro_maker_name() -> TokenStream2 {
    let trait_name = proc_macro2::Ident::new("MacroMarker", proc_macro2::Span::call_site());

    quote::quote! {
        auto_delegate::#trait_name
    }
}


pub fn expand_macro_marker_generics(trait_ident: Ident) -> TokenStream2 {
    let str = ident_to_lower_string(trait_ident);

    let mut chars = str.chars();

    const GENERICS_COUNT: usize = 30;

    let params = (0..GENERICS_COUNT)
        .map(|_| {
            let c = chars.next().unwrap_or(' ');
            quote::quote!(#c,)
        });

    quote::quote!(#(#params)*)
}