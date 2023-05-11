use syn::__private::TokenStream2;

pub fn expand_macro_maker_ident(maker_no: usize) -> TokenStream2 {
    let mod_path = proc_macro2::Ident::new(
        format!("maker{}", maker_no).as_str(),
        proc_macro2::Span::call_site(),
    );
    let trait_name = proc_macro2::Ident::new(
        format!("MacroMarker{}", maker_no).as_str(),
        proc_macro2::Span::call_site(),
    );

    quote::quote! {
        define::#mod_path::#trait_name
    }
}
