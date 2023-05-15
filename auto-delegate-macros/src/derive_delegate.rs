use proc_macro::TokenStream;
use proc_macro2::Span;

use syn::{ItemEnum, ItemStruct};

use crate::derive_delegate::derive_enum::try_expand_derive_enum;
use crate::derive_delegate::derive_struct::try_expand_derive_delegate_struct;

mod by_fields;
mod derive_struct;
pub mod derive_enum;

pub fn expand_delegate(input: TokenStream) -> proc_macro2::TokenStream {

    if let Ok(item_struct) = syn::parse::<ItemStruct>(input.clone()){
        return expand_struct(item_struct)
    }

    if let Ok(item_enum) = syn::parse::<syn::ItemEnum>(input){
        return expand_enum(item_enum)
    }


    return syn::Error::new(Span::call_site(), "").to_compile_error()
}

fn expand_struct(item_struct: ItemStruct) -> proc_macro2::TokenStream {
    match try_expand_derive_delegate_struct(item_struct) {
        Ok(output) => output,
        Err(error) => error.to_compile_error(),
    }
}


fn expand_enum(item_enum: ItemEnum) -> proc_macro2::TokenStream {
    match try_expand_derive_enum(&item_enum) {
        Ok(output) => output,
        Err(error) => error.to_compile_error(),
    }
}
