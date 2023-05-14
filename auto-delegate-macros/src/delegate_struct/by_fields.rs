use proc_macro2::Ident;
use syn::{Field, Fields};

use crate::attribute::{find_by_attribute, trait_names};

pub struct ByField {
    field_name: Ident,
    field_ty: syn::Type,
    trait_names: Vec<Ident>,
}


impl ByField {
    pub fn field_name_ref(&self) -> &Ident {
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
            find_by_attribute(&field.attrs).and_then(|by_attr| trait_names(&by_attr))
        {
            Some(ByField {
                field_name: field.ident.unwrap(),
                field_ty: field.ty,
                trait_names,
            })
        } else {
            self.next()
        }
    }
}


