use proc_macro2::Ident;
use quote::quote;
use syn::__private::TokenStream2;
use syn::TraitItemFn;

use crate::trait_item::trait_fn_inputs::TraitFnInputs;

pub struct TraitFnMeta(TraitItemFn);


impl TraitFnMeta {
    pub const fn new(item_fn: TraitItemFn) -> Self {
        Self(item_fn)
    }


    pub fn expand_fn(&mut self) -> syn::Result<TokenStream2> {
        let fn_inputs = TraitFnInputs::new(self.0.sig.inputs.clone());
        let delegate = fn_inputs.expand_delegate_method(self.fn_name());

        self.0.default.replace(syn::parse2(quote!({
            #delegate
        })).unwrap());

        let f = &self.0;
        Ok(quote::quote!{
            #[inline(always)]
            #f
        })
    }


    fn fn_name(&self) -> &Ident {
        &self.0.sig.ident
    }
}

