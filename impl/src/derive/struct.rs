use proc_macro2::Ident;
use syn::{Attribute, Fields, Generics, ItemStruct, Type};
use syn::__private::TokenStream2;
use syn::Fields::Unnamed;

use crate::attribute::{find_to_attribute, syn_error_not_found_fields, syn_error_not_found_trait_names, trait_names};
use crate::delegatable::delegatable_ident_with_generics;
use crate::derive::by_fields::{ByField, ByFields};

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
        impl_delegatable(
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
        .map(|by_field| impl_delegatable_with_fields(struct_name, by_field, &item_struct.generics));

    Ok(quote::quote! {
        #(#expand_impl_methods)*
    })
}


fn impl_delegatable_with_fields(
    struct_name: &Ident,
    by_field: ByField,
    generics: &Generics,
) -> proc_macro2::TokenStream {
    impl_delegatable(
        struct_name,
        by_field.field_name_ref(),
        by_field.field_ty_ref(),
        by_field.trait_names_ref(),
        generics,
    )
}


fn impl_delegatable(
    struct_name: &Ident,
    delegate_field_name: &TokenStream2,
    delegate_filed_ty: &Type,
    trait_names: &[Ident],
    generics: &Generics,
)
    -> TokenStream2 {
    let (_, type_params, _) = &generics.split_for_impl();
    let where_bound = generics.where_clause.as_ref();

    let expand = trait_names
        .iter()
        .map(|trait_name| {
            let delegatable_ident = delegatable_ident_with_generics(trait_name.clone());

            quote::quote! {
                impl #generics #delegatable_ident for #struct_name #type_params #where_bound{
                    type A = #delegate_filed_ty;
                    type B = #delegate_filed_ty;
                    type C = #delegate_filed_ty;
                    type D = #delegate_filed_ty;
                    type E = #delegate_filed_ty;
                    type F = #delegate_filed_ty;
                    type G = #delegate_filed_ty;
                    type H = #delegate_filed_ty;
                    type I = #delegate_filed_ty;
                    type J = #delegate_filed_ty;
                    type K = #delegate_filed_ty;
                    type L = #delegate_filed_ty;

                    #[inline(always)]
                    fn delegate_by_owned(self) -> auto_delegate::Delegates<Self::A, Self::B, Self::C, Self::D, Self::E, Self::F, Self::G, Self::H, Self::I, Self::J, Self::K, Self::L>{
                        auto_delegate::Delegates(Some(self.#delegate_field_name), None, None, None, None, None, None, None, None, None, None, None)
                    }

                    #[inline(always)]
                    fn delegate_by_ref(&self) -> auto_delegate::Delegates<&Self::A, &Self::B, &Self::C, &Self::D, &Self::E, &Self::F, &Self::G, &Self::H, &Self::I, &Self::J, &Self::K, &Self::L>{
                        auto_delegate::Delegates(Some(&self.#delegate_field_name), None, None, None, None, None, None, None, None, None, None, None)
                    }

                    #[inline(always)]
                    fn delegate_by_mut(&mut self) -> auto_delegate::Delegates<&mut Self::A, &mut Self::B, &mut Self::C, &mut Self::D, &mut Self::E, &mut Self::F, &mut Self::G, &mut Self::H, &mut Self::I, &mut Self::J, &mut Self::K, &mut Self::L>{
                        auto_delegate::Delegates(Some(&mut self.#delegate_field_name), None, None, None, None, None, None, None, None, None, None, None)
                    }
                }
            }
        });

    quote::quote!(#(#expand)*)
}