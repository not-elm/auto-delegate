use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use syn::{Attribute, Field, Fields};

pub struct ByField {
    field_name: Ident,
    field_ty: syn::Type,
    _trait_names: Vec<Ident>,
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
        &self._trait_names
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
            find_by_attribute(&field).and_then(|by_attr| trait_names(&by_attr))
        {
            Some(ByField {
                field_name: field.ident.unwrap(),
                field_ty: field.ty,
                _trait_names: trait_names,
            })
        } else {
            self.next()
        }
    }
}


fn find_by_attribute(field: &Field) -> Option<Attribute> {
    field
        .attrs
        .iter()
        .find(|attr| {
            attr.meta
                .path()
                .is_ident("by")
        })
        .cloned()
}


fn trait_names(by_attr: &Attribute) -> Option<Vec<Ident>> {
    let mut tokens = TokenStream::new();
    by_attr.to_tokens(&mut tokens);

    let mut trait_names: Vec<Ident> = Vec::new();
    by_attr
        .parse_nested_meta(|meta| {
            meta.path
                .segments
                .into_iter()
                .for_each(|s| trait_names.push(s.ident));
            Ok(())
        })
        .ok()?;
    if trait_names.is_empty() {
        None
    } else {
        Some(trait_names)
    }
}
