use proc_macro2::Ident;
use syn::__private::TokenStream2;
use syn::ItemEnum;

use crate::attribute::{find_to_attribute, syn_error_must_attach_to_attribute, trait_names};
use crate::macro_marker::expand_macro_maker_ident;

pub fn try_expand_derive_enum(item_enum: &ItemEnum) -> syn::Result<TokenStream2> {
    let delegate_by_ref = expand_impl_delegate_by_ref(item_enum);
    let delegate_by_mut = expand_impl_delegate_by_mut(item_enum);

    let to_attr = find_to_attribute(&item_enum.attrs).ok_or(syn_error_must_attach_to_attribute())?;

    let trait_names = trait_names(&to_attr).ok_or(syn_error_must_attach_to_attribute())?;

    let expand_impls = trait_names
        .into_iter()
        .map(|trait_name| {
            expand_impl_macro_marker(trait_name, item_enum, &delegate_by_ref, &delegate_by_mut)
        });

    Ok(quote::quote! {
        #(#expand_impls)*
    })
}


fn expand_impl_macro_marker(
    trait_name: Ident,
    item_enum: &ItemEnum,
    delegate_by_ref: &TokenStream2,
    delegate_by_mut: &TokenStream2,
) -> TokenStream2 {
    let marker_ident = expand_macro_maker_ident(trait_name.clone());
    let enum_name = &item_enum.ident;

    let generics = &item_enum.generics;

    quote::quote! {
        impl #generics #marker_ident for #enum_name #generics{
            type DelegateType = dyn #trait_name #generics;

            #delegate_by_ref

            #delegate_by_mut
        }
    }
}


fn expand_impl_delegate_by_ref(item_enum: &ItemEnum) -> TokenStream2 {
    let patterns = pattern_match_fields(item_enum);

    quote::quote! {
        fn delegate_by_ref<'a, Output: 'a>(
            &'a self,
            f: impl FnOnce(&'a Self::DelegateType) -> Output) -> Output{

            match self{
                 #patterns
            }
        }
    }
}


fn expand_impl_delegate_by_mut(item_enum: &ItemEnum) -> TokenStream2 {
    let patterns = pattern_match_fields(item_enum);

    quote::quote! {
        fn delegate_by_mut<'a, Output: 'a>(
            &'a mut self,
            f: impl FnOnce(&'a mut Self::DelegateType) -> Output) -> Output{
            match self{
                #patterns
            }
        }
    }
}


fn pattern_match_fields(item_enum: &ItemEnum) -> TokenStream2 {
    let matches = item_enum
        .variants
        .iter()
        .map(|v| {
            let ident = &v.ident;

            quote::quote! {
                Self::#ident(v) => f(v),
            }
        });

    quote::quote! {
        #(#matches)*
    }
}
