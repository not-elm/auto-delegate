use proc_macro2::Ident;
use syn::{Attribute, Fields, Generics, ItemStruct, Type};
use syn::__private::TokenStream2;
use syn::Fields::Unnamed;

use crate::attribute::{find_to_attribute, syn_error_not_found_fields, syn_error_not_found_trait_names, trait_names};
use crate::derive_delegate::by_fields::{ByField, ByFields};
use crate::macro_marker::expand_macro_maker_ident;
use crate::syn::syn_generics::expand_where_bound;

pub fn try_expand_derive_delegate_struct(item_struct: ItemStruct) -> syn::Result<proc_macro2::TokenStream> {
    let to_attr = find_to_attribute(&item_struct.attrs);

    if let Some(to_attr) = to_attr {
        impl_macro_markers_with_new_type(&item_struct, &to_attr)
    } else {
        impl_macro_markers_with_named_fields(item_struct)
    }
}


fn impl_macro_markers_with_new_type(
    item: &ItemStruct,
    to_attr: &Attribute,
) -> syn::Result<TokenStream2> {
    let trait_names = trait_names(to_attr)
        .ok_or(syn_error_not_found_trait_names())?;

    let delegate_field_name = quote::quote!(0);
    let delegate_field_ty = parse_new_type_ty(item.fields.clone())
        .ok_or(syn_error_not_found_fields())?;

    Ok(
        impl_macro_marker(
            &item.ident,
            &delegate_field_name,
            &delegate_field_ty,
            &trait_names,
            &item.generics,
        )
    )
}


fn parse_new_type_ty(
    fields: Fields
) -> Option<Type> {
    if let Unnamed(un_named) = fields {
        Some(un_named
            .unnamed
            .first()?
            .ty
            .clone()
        )
    } else {
        None
    }
}


fn impl_macro_markers_with_named_fields(item_struct: ItemStruct) -> syn::Result<TokenStream2> {
    let struct_name = &item_struct.ident;

    let expand_impl_methods = ByFields::new(item_struct.fields)
        .map(|by_field| impl_macro_marker_with_named_field(struct_name, by_field, &item_struct.generics));


    Ok(quote::quote! {
            #(#expand_impl_methods)*
        })
}


fn impl_macro_marker_with_named_field(
    struct_name: &Ident,
    by_field: ByField,
    generics: &Generics,
) -> proc_macro2::TokenStream {
    impl_macro_marker(
        struct_name,
        by_field.field_name_ref(),
        by_field.field_ty_ref(),
        by_field.trait_names_ref(),
        generics,
    )
}


fn impl_macro_marker(
    struct_name: &Ident,
    delegate_field_name: &TokenStream2,
    delegate_filed_ty: &Type,
    trait_names: &[Ident],
    generics: &Generics,
)
    -> TokenStream2 {
    let (_, type_params, _) = &generics.split_for_impl();
    let where_bound = expand_where_bound(generics);

    let expand = trait_names
        .iter()
        .map(|trait_name| {
            let macro_marker_ident = expand_macro_maker_ident(trait_name.clone());

            quote::quote! {
                impl #generics #macro_marker_ident for #struct_name #type_params #where_bound{
                    type DelegateType = #delegate_filed_ty;

                    fn delegate_by_ref<'delegate_lifetime, Output: 'delegate_lifetime>(&'delegate_lifetime self, f: impl FnOnce(&'delegate_lifetime Self::DelegateType) -> Output) -> Output{
                        f(&self.#delegate_field_name)
                    }

                    fn delegate_by_mut<'delegate_lifetime, Output:'delegate_lifetime>(&'delegate_lifetime mut self, f: impl FnOnce(&'delegate_lifetime mut Self::DelegateType) -> Output) -> Output{
                        f(&mut self.#delegate_field_name)
                    }
                }
            }
        });

    quote::quote!(#(#expand)*)
}