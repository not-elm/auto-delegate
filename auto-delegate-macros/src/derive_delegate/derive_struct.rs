use syn::{Generics, ItemStruct};

use crate::derive_delegate::by_fields::{ByField, ByFields};
use crate::macro_marker::expand_macro_maker_ident;
use crate::syn::syn_generics::{expand_generics_with_brackets_without_bound, expand_where_bound};

pub fn try_expand_derive_delegate_struct(item_struct: ItemStruct) -> syn::Result<proc_macro2::TokenStream> {
    let struct_name = &item_struct.ident;

    let expand_impl_methods = ByFields::new(item_struct.fields)
        .map(|by_field| impl_method_by_delegate(struct_name, by_field, &item_struct.generics));


    Ok(quote::quote! {
        #(#expand_impl_methods)*
    })
}


fn impl_method_by_delegate(
    struct_name: &syn::Ident,
    by_field: ByField,
    generics: &Generics,
) -> proc_macro2::TokenStream {
    let delegate_field_name = by_field.field_name_ref();
    let delegate_filed_ty = by_field.field_ty_ref();

    let generics_param = expand_generics_with_brackets_without_bound(generics);
    let where_bound = expand_where_bound(generics);

    let expand = by_field
        .trait_names_ref()
        .iter()
        .map(|trait_name| {
            let macro_marker_ident = expand_macro_maker_ident(trait_name.clone());

            quote::quote! {
                impl #generics #macro_marker_ident for #struct_name #generics_param #where_bound{
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
