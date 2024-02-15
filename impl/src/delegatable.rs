use proc_macro2::Ident;
use syn::__private::TokenStream2;

use crate::ident::ident_to_lower_string;

pub fn delegatable_ident_with_generics(trait_ident: Ident) -> TokenStream2 {
    let name = delegatable_name();
    let generics = delegatable_generics(trait_ident);

    quote::quote!(#name<#generics>)
}


fn delegatable_name() -> TokenStream2 {
    let trait_name = proc_macro2::Ident::new("Delegatable", proc_macro2::Span::call_site());

    quote::quote! {
        auto_delegate::#trait_name
    }
}


fn delegatable_generics(trait_ident: Ident) -> TokenStream2 {
    let str = ident_to_lower_string(trait_ident);

    let mut chars = str.chars();

    const GENERICS_COUNT: usize = 30;

    let generic_chars = (0..GENERICS_COUNT)
        .map(|_| {
            let c = chars.next().unwrap_or(' ');
            quote::quote!(#c)
        })
        .collect::<Vec<TokenStream2>>();

    let mut params = Vec::with_capacity(generic_chars.len() * 2);
    for c in generic_chars {
        params.push(c);
        params.push(quote::quote!(,))
    }

    params.pop();

    quote::quote!(#(#params)*)
}