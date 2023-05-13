use syn::__private::TokenStream2;

pub fn expand_macro_maker_ident() -> TokenStream2 {
    let trait_name = proc_macro2::Ident::new("MacroMarker", proc_macro2::Span::call_site());

    quote::quote! {
        auto_delegate::#trait_name
    }
}

