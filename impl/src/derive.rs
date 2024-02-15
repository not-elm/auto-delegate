use proc_macro::TokenStream;

use proc_macro2::Span;
use syn::{ItemEnum, ItemStruct};

use crate::derive::r#enum::try_expand_derive_enum;
use crate::derive::r#struct::try_expand_derive_delegate_struct;

mod by_fields;
mod r#struct;
mod r#enum;

pub fn expand_derive_delegate(input: TokenStream) -> proc_macro2::TokenStream {
    if let Ok(item_struct) = syn::parse::<ItemStruct>(input.clone()) {
        return expand_struct(item_struct);
    }

    if let Ok(item_enum) = syn::parse::<ItemEnum>(input) {
        return expand_enum(item_enum);
    }


    syn::Error::new(Span::call_site(), "Inherits must be Enum or Structs.").to_compile_error()
}


fn expand_struct(item_struct: ItemStruct) -> proc_macro2::TokenStream {
    try_expand_derive_delegate_struct(item_struct)
        .unwrap_or_else(|error| error.to_compile_error())
}


fn expand_enum(item_enum: ItemEnum) -> proc_macro2::TokenStream {
    try_expand_derive_enum(&item_enum)
        .unwrap_or_else(|error| error.to_compile_error())
}
