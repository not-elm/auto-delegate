use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use syn::{Attribute, Field, Fields};

pub struct ByFieldIdent {
    field_name: Ident,
    trait_name: Ident,
}


impl ByFieldIdent {
    pub fn field_name_ref(&self) -> &Ident {
        &self.field_name
    }


    pub fn trait_name_ref(&self) -> &Ident {
        &self.trait_name
    }
}


pub struct ByFields {
    fields: Box<dyn Iterator<Item = Field>>,
}


impl ByFields {
    pub fn new(fields: Fields) -> Self {
        Self {
            fields: Box::new(fields.into_iter()),
        }
    }
}


impl Iterator for ByFields {
    type Item = ByFieldIdent;

    fn next(&mut self) -> Option<Self::Item> {
        let field = self.fields.next()?;

        if let Some(trait_names) =
            find_by_attribute(&field).and_then(|by_attr| trait_names(&by_attr))
        {
            Some(ByFieldIdent {
                field_name: field.ident.unwrap(),
                trait_name: trait_names //TODO: 複数のトレイトに対応
                    .get(0)
                    .unwrap()
                    .clone(),
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
