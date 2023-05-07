use proc_macro::TokenStream;

use proc_macro2::Ident;
use syn::ItemTrait;
use syn::__private::TokenStream2;
use syn::spanned::Spanned;

pub fn expand_delegate_trait(_attr: TokenStream, input: TokenStream) -> TokenStream2 {
    match try_expand_delegate_trait(input) {
        Ok(output) => output,
        Err(error) => error.to_compile_error(),
    }
}


fn try_expand_delegate_trait(input: TokenStream) -> syn::Result<TokenStream2> {
    let input_trait = syn::parse::<syn::ItemTrait>(input)?;

    Ok(expand_impl_macro(&input_trait))
}


fn expand_impl_macro(item: &ItemTrait) -> TokenStream2 {
    let trait_ident = &item.ident;
    let trait_name = trait_ident
        .clone()
        .to_string();
    let macro_name = format!("impl_delegate_{}", trait_name.to_lowercase());
    let macro_name = Ident::new(&macro_name, proc_macro2::Span::call_site());

    let span = item.span();
    quote::quote_spanned! { span =>
        #[macro_export]
        macro_rules! #macro_name{
            ($struct_name: ident) => {
                impl #trait_ident for $struct_name{

                }
            };
        }
    }
}
