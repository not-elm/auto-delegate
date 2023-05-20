use proc_macro2::Ident;
use syn::{Field, Fields};
use syn::__private::TokenStream2;

use crate::attribute::{find_to_attribute, trait_names};
use crate::ident::ident_to_token_stream;

pub struct ByField {
    field_name: TokenStream2,
    field_ty: syn::Type,
    trait_names: Vec<Ident>,
}


impl ByField {
    pub fn field_name_ref(&self) -> &TokenStream2 {
        &self.field_name
    }


    pub fn field_ty_ref(&self) -> &syn::Type {
        &self.field_ty
    }


    #[allow(unused)]
    pub fn trait_names_ref(&self) -> &Vec<Ident> {
        &self.trait_names
    }
}


pub struct ByFields {
    fields: Box<dyn Iterator<Item=Field>>,
}


impl ByFields {
    pub fn new(fields: Fields) -> Self {
        Self {
            fields: Box::new(fields.into_iter()),
        }
    }
}


impl Iterator for ByFields {
    type Item = ByField;

    fn next(&mut self) -> Option<Self::Item> {
        let field = self.fields.next()?;

        if let Some(trait_names) =
            find_to_attribute(&field.attrs).and_then(|by_attr| trait_names(&by_attr))
        {
            Some(ByField {
                field_name: ident_to_field_name(&field.ident),
                field_ty: field.ty,
                trait_names,
            })
        } else {
            self.next()
        }
    }
}


fn ident_to_field_name(
    ident: &Option<Ident>
) -> TokenStream2 {
    if let Some(ident) = ident {
        ident_to_token_stream(ident)
    } else {
        quote::quote!(#0)
    }
}