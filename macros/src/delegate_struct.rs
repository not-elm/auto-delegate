use proc_macro::TokenStream;

use proc_macro2::Ident;
use syn::ItemStruct;

use crate::delegate_struct::by_fields::{ByField, ByFields};
use crate::macro_marker::expand_macro_maker_ident;

mod by_fields;

pub fn expand_delegate(input: TokenStream) -> proc_macro2::TokenStream {
    match try_expand_delegate(input) {
        Ok(output) => output,
        Err(error) => error.to_compile_error(),
    }
}


fn try_expand_delegate(input: TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    let item_struct = syn::parse::<ItemStruct>(input)?;
    let struct_name = item_struct.clone().ident;

    let expand_impl_methods = ByFields::new(item_struct.fields)
        .take(1)
        .map(|by_field| impl_method_by_delegate(&struct_name, by_field));


    Ok(quote::quote! {
        #(#expand_impl_methods)*
    })
}


fn impl_method_by_delegate(
    struct_name: &Ident,
    by_field: ByField,
) -> proc_macro2::TokenStream {
    let delegate_field_name = by_field.field_name_ref();
    let delegate_filed_ty = by_field.field_ty_ref();
    let macro_marker_ident = expand_macro_maker_ident();

    quote::quote! {
        impl #macro_marker_ident for #struct_name{
            type DelegateType = #delegate_filed_ty;

            fn delegate_by_ref(&self) -> &Self::DelegateType{
                &self.#delegate_field_name
            }

            fn delegate_by_mut(&mut self) -> &mut Self::DelegateType{
                &mut self.#delegate_field_name
            }
        }
    }
}


// fn crate_path(span: proc_macro::Span) -> Option<TokenStream2> {
//     if span
//         .source_file()
//         .path()
//         .iter()
//         .any(|p| p == "tests")
//     {
//         None
//     } else {
//         Some(quote::quote! {$crate::})
//     }
// }
