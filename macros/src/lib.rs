#![feature(iter_intersperse)]

use proc_macro::TokenStream;

use crate::delegate::expand_delegate;
use crate::delegate_trait::expand_delegate_trait;

mod delegate;
mod delegate_trait;
mod ident;
mod trait_fn_inputs;
mod trait_fn_iter;
mod trait_fn_meta;

#[proc_macro_attribute]
pub fn delegate_trait(attr: TokenStream, input: TokenStream) -> TokenStream {
    let output = expand_delegate_trait(attr, input.clone());
    expand_unit(input, output)
}

#[proc_macro_attribute]
pub fn delegate(attr: TokenStream, input: TokenStream) -> TokenStream {
    let output = expand_delegate(attr, input.clone());
    expand_unit(input, output)
}

fn expand_unit(input: TokenStream, output: proc_macro2::TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let expand = quote::quote! {
        #input
        #output
    };

    expand.into()
}
