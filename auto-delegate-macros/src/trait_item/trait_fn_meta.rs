use proc_macro2::Ident;
use syn::{ReturnType, TraitItemFn};
use syn::__private::TokenStream2;

use crate::syn_type::expand_syn_type;
use crate::trait_item::trait_fn_inputs::TraitFnInputs;

pub struct TraitFnMeta(TraitItemFn);


impl TraitFnMeta {
    pub const fn new(item_fn: TraitItemFn) -> Self {
        Self(item_fn)
    }


    pub fn expand_fn(&self) -> syn::Result<TokenStream2> {
        let fn_name = self.fn_name();
        let output = self.output();
        let fn_inputs = TraitFnInputs::new(self.0.sig.inputs.clone());

        let args = fn_inputs.expand_args()?;
        let inputs = fn_inputs.expand_inputs();
        let delegate = fn_inputs.expand_delegate_method();

        Ok(quote::quote! {
            fn #fn_name(#args) #output{
                #delegate.#fn_name(#inputs)
            }
        })
    }


    fn fn_name(&self) -> &Ident {
        &self.0.sig.ident
    }


    fn output(&self) -> Option<TokenStream2> {
        if let ReturnType::Type(_, type_path) = &self.0.sig.output {
            expand_syn_type(type_path).map(|ty| quote::quote! {-> #ty})
        } else {
            None
        }
    }
}

