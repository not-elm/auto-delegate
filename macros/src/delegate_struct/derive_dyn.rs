use proc_macro::TokenStream;

use proc_macro2::Ident;
use syn::ItemStruct;

use crate::delegate_struct::by_fields::{ByField, ByFields};
use crate::macro_marker::expand_dyn_macro_maker_ident;

pub fn expand_derive_dyn_delegate(input: TokenStream) -> proc_macro2::TokenStream {
    match try_expand_delegate(input) {
        Ok(output) => output,
        Err(error) => error.to_compile_error(),
    }
}


fn try_expand_delegate(input: TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    let item_struct = syn::parse::<ItemStruct>(input)?;
    let struct_name = item_struct.clone().ident;

    let expand_impl_methods = ByFields::new(item_struct.fields).map(|by_field| {
        let e = impl_method_by_dyn_delegate(&struct_name, by_field);
        dbg!(e.clone());
        e
    });


    Ok(quote::quote! {
        #(#expand_impl_methods)*
    })
}


fn impl_method_by_dyn_delegate(struct_name: &Ident, by_field: ByField) -> proc_macro2::TokenStream {
    let delegate_field_name = by_field.field_name_ref();

    let expand_impls = by_field
        .trait_names_ref()
        .iter()
        .map(|trait_name| {
            let macro_marker_ident = expand_dyn_macro_maker_ident(trait_name);

            quote::quote! {
                impl #macro_marker_ident for #struct_name{

                    fn delegate_by_ref(&self) -> Box<& dyn #trait_name>{
                        self.#delegate_field_name
                    }

                    fn delegate_by_mut(&mut self) -> Box<&mut dyn #trait_name>{
                        self.#delegate_field_name
                    }
                }
            }
        });

    quote::quote! {
        #(#expand_impls)*
    }
}
