use syn::TraitItem;

use crate::trait_item::trait_fn_meta::TraitFnMeta;

pub struct TraitFnIter {
    items: Box<dyn Iterator<Item = TraitItem>>,
}


impl TraitFnIter {
    pub fn new(items: Vec<TraitItem>) -> Self {
        Self {
            items: Box::new(items.into_iter()),
        }
    }
}


impl Iterator for TraitFnIter {
    type Item = TraitFnMeta;

    fn next(&mut self) -> Option<Self::Item> {
        if let TraitItem::Fn(item_fun) = self.items.next()? {
            Some(TraitFnMeta::new(item_fun))
        } else {
            self.next()
        }
    }
}
