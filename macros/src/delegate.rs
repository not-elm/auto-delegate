use proc_macro::TokenStream;

use syn::ItemStruct;
use syn::__private::TokenStream2;
use syn::parse::Parser;

use crate::ident::generate_delegate_impl_macro_name;

pub fn expand_delegate(args: TokenStream, input: TokenStream) -> proc_macro2::TokenStream {
    match try_expand_delegate(args, input) {
        Ok(output) => output,
        Err(error) => error.to_compile_error(),
    }
}

fn try_expand_delegate(
    args: TokenStream,
    input: TokenStream,
) -> syn::Result<proc_macro2::TokenStream> {
    let item_struct = syn::parse::<ItemStruct>(input)?;
    let args = TokenStream2::from(args);

    let path_args = <syn::punctuated::Punctuated<syn::Path, syn::Token![,]>>::parse_terminated
        .parse(args.into())?;

    let trait_ident = path_args
        .first()
        .unwrap()
        .segments
        .first()
        .unwrap()
        .clone()
        .ident;


    let struct_name = item_struct.ident;
    let macro_name = generate_delegate_impl_macro_name(trait_ident);
    Ok(quote::quote! {
        #macro_name!(#struct_name);
    })
}
