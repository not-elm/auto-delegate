use proc_macro::TokenStream;

use proc_macro2::Ident;
use syn::ItemTrait;
use syn::__private::TokenStream2;
use syn::spanned::Spanned;

use crate::trait_fn_iter::TraitFnIter;

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
    let trait_name = trait_ident
        .clone()
        .to_string();
    let macro_name = format!("impl_delegate_{}", trait_name.to_lowercase());
    let macro_name = Ident::new(&macro_name, proc_macro2::Span::call_site());

    let trait_fn = TraitFnIter::new(item.clone().items).filter_map(|meta| meta.expand_fn().ok());

    let span = item.span();
    Ok(quote::quote_spanned! { span =>
        #[macro_export]
        macro_rules! #macro_name{
            ($struct_name: ident) => {
                impl #trait_ident for $struct_name{
                    #(#trait_fn)*
                }
            };
        }
    })
}
