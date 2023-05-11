use syn::Type;
use syn::__private::TokenStream2;

pub fn expand_macro_maker_ident() -> TokenStream2 {
    let trait_name = proc_macro2::Ident::new("MacroMarker", proc_macro2::Span::call_site());

    quote::quote! {
        define::#trait_name
    }
}


pub fn expand_dyn_macro_maker_ident(trait_ident: &proc_macro2::Ident) -> TokenStream2 {
    let trait_name = proc_macro2::Ident::new("MacroDynMarker", proc_macro2::Span::call_site());

    quote::quote! {
        define::#trait_name<Box<dyn #trait_ident>>
    }
}
