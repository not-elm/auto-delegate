use proc_macro2::Ident;
use quote::quote;
use syn::__private::TokenStream2;
use syn::ItemEnum;

use crate::attribute::{find_to_attribute, syn_error_not_found_to_attribute, syn_error_not_found_trait_names, trait_names};
use crate::delegatable::delegatable_ident_with_generics;

pub fn try_expand_derive_enum(item_enum: &ItemEnum) -> syn::Result<TokenStream2> {
    let delegate_by_owned = expand_impl_delegate_by_owned(item_enum);
    let delegate_by_ref = expand_impl_delegate_by_ref(item_enum);
    let delegate_by_mut = expand_impl_delegate_by_mut(item_enum);
    let to_attr = find_to_attribute(&item_enum.attrs).ok_or(syn_error_not_found_to_attribute())?;
    let trait_names = trait_names(&to_attr).ok_or(syn_error_not_found_trait_names())?;
    let expand_impls = trait_names
        .into_iter()
        .map(|trait_name| {
            expand_impl_delegatable(trait_name, item_enum, &delegate_by_owned, &delegate_by_ref, &delegate_by_mut)
        });

    Ok(quote::quote! {
        #(#expand_impls)*
    })
}


fn expand_impl_delegatable(
    trait_name: Ident,
    item_enum: &ItemEnum,
    delegate_by_owned: &TokenStream2,
    delegate_by_ref: &TokenStream2,
    delegate_by_mut: &TokenStream2,
) -> TokenStream2 {
    let marker_ident = delegatable_ident_with_generics(trait_name.clone());
    let enum_name = &item_enum.ident;
    let generics = &item_enum.generics;
    let (_, types, _) = generics.split_for_impl();

    let bounds = &generics.where_clause;
    let (a, b, c, d, e, f, g, h, i, j, k, l) = variant_types(item_enum);

    quote::quote! {
        impl #generics #marker_ident for #enum_name #types
            #bounds
        {
            type A = #a;
            type B = #b;
            type C = #c;
            type D = #d;
            type E = #e;
            type F = #f;
            type G = #g;
            type H = #h;
            type I = #i;
            type J = #j;
            type K = #k;
            type L = #l;

            #delegate_by_owned
            #delegate_by_ref
            #delegate_by_mut
        }
    }
}


fn expand_impl_delegate_by_owned(item_enum: &ItemEnum) -> TokenStream2 {
    let patterns = pattern_match_fields(item_enum);

    quote::quote! {
        #[inline(always)]
        fn delegate_by_owned(self) -> auto_delegate::Delegates<Self::A, Self::B, Self::C, Self::D, Self::E, Self::F, Self::G, Self::H, Self::I, Self::J, Self::K, Self::L>{
            match self{
                 #patterns
            }
        }
    }
}


fn expand_impl_delegate_by_ref(item_enum: &ItemEnum) -> TokenStream2 {
    let patterns = pattern_match_fields(item_enum);

    quote::quote! {
        #[inline(always)]
        fn delegate_by_ref(&self) -> auto_delegate::Delegates<&Self::A, &Self::B, &Self::C, &Self::D, &Self::E, &Self::F, &Self::G, &Self::H, &Self::I, &Self::J, &Self::K, &Self::L>{
            match self{
                 #patterns
            }
        }
    }
}


fn expand_impl_delegate_by_mut(item_enum: &ItemEnum) -> TokenStream2 {
    let patterns = pattern_match_fields(item_enum);

    quote::quote! {
        #[inline(always)]
        fn delegate_by_mut(&mut self) -> auto_delegate::Delegates<&mut Self::A, &mut Self::B, &mut Self::C, &mut Self::D, &mut Self::E, &mut Self::F, &mut Self::G, &mut Self::H, &mut Self::I, &mut Self::J, &mut Self::K, &mut Self::L>{
            match self{
                 #patterns
            }
        }
    }
}


fn variant_types(item_enum: &ItemEnum) -> (TokenStream2, TokenStream2, TokenStream2, TokenStream2, TokenStream2, TokenStream2, TokenStream2, TokenStream2, TokenStream2, TokenStream2, TokenStream2, TokenStream2) {
    let mut types = item_enum
        .variants
        .iter()
        .map(|v| v.fields.iter().next().unwrap().ty.clone())
        .map(|t| quote!(#t));

    let a = types.next().unwrap();
    let a2 = a.clone();
    let mut t = || {
        types.next().unwrap_or(a2.clone())
    };
    (a, t(), t(), t(), t(), t(), t(), t(), t(), t(), t(), t())
}


fn pattern_match_fields(item_enum: &ItemEnum) -> TokenStream2 {
    let matches = item_enum
        .variants
        .iter()
        .enumerate()
        .map(|(i, v)| {
            let ident = &v.ident;

            match i {
                0 => quote::quote! {
                    Self::#ident(v) => auto_delegate::Delegates(Some(v), None, None, None, None, None, None, None, None, None, None, None),
                },
                1 => quote::quote! {
                    Self::#ident(v) => auto_delegate::Delegates(None, Some(v), None, None, None, None, None, None, None, None, None, None),
                },
                2 => quote::quote! {
                    Self::#ident(v) => auto_delegate::Delegates(None, None, Some(v),  None, None, None, None, None, None, None, None, None),
                },
                3 => quote::quote! {
                    Self::#ident(v) => auto_delegate::Delegates(None, None, None, Some(v), None, None, None, None, None, None, None, None),
                },
                4 => quote::quote! {
                    Self::#ident(v) => auto_delegate::Delegates(None, None, None, None, Some(v), None, None, None, None, None, None, None),
                },
                5 => quote::quote! {
                    Self::#ident(v) => auto_delegate::Delegates(None, None, None, None, None, Some(v), None, None, None, None, None, None),
                },
                6 => quote::quote! {
                    Self::#ident(v) => auto_delegate::Delegates(None, None, None, None, None, None, Some(v), None, None, None, None, None),
                },
                7 => quote::quote! {
                    Self::#ident(v) => auto_delegate::Delegates(None, None, None, None, None, None, None, Some(v), None, None, None, None),
                },
                8 => quote::quote! {
                    Self::#ident(v) => auto_delegate::Delegates(None, None, None, None, None, None, None, None, Some(v), None, None, None),
                },
                9 => quote::quote! {
                    Self::#ident(v) => auto_delegate::Delegates(None, None, None, None, None, None, None, None, None, Some(v), None, None),
                },
                10 => quote::quote! {
                    Self::#ident(v) => auto_delegate::Delegates(None, None, None, None, None, None, None, None, None, None, Some(v), None),
                },
                11 => quote::quote! {
                    Self::#ident(v) => auto_delegate::Delegates(None, None, None, None, None, None, None, None, None, None, None, Some(v)),
                },
                _ => panic!("not reachable")
            }
        });

    quote::quote! {
        #(#matches)*
    }
}
