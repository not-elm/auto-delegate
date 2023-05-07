use proc_macro2::Ident;
use syn::__private::TokenStream2;
use syn::{ReturnType, TraitItemFn, Type, TypePath};


use crate::trait_item::trait_fn_inputs::TraitFnInputs;

pub struct TraitFnMeta(TraitItemFn);


impl TraitFnMeta {
    pub const fn new(item_fn: TraitItemFn) -> Self {
        Self(item_fn)
    }


    pub fn expand_fn(&self) -> syn::Result<TokenStream2> {
        let fn_name = self.fn_name();
        let output_ty = self.output_ty();
        let fn_inputs = TraitFnInputs::new(self.0.sig.inputs.clone());

        let args = fn_inputs.expand_args()?;
        let inputs = fn_inputs.expand_inputs();
        Ok(quote::quote! {
            // TODO 引数やレシーバ
            fn #fn_name(#args) -> #output_ty{
                self.$delegate_field.#fn_name(#inputs)
            }
        })
    }


    fn fn_name(&self) -> &Ident {
        &self.0.sig.ident
    }


    fn output_ty(&self) -> Option<&Ident> {
        if let ReturnType::Type(_, type_path) = &self.0.sig.output {
            return_ty(type_path)
        } else {
            None
        }
    }
}


fn return_ty(ty: &Type) -> Option<&Ident> {
    match ty {
        Type::Path(path) => Some(return_ty_path(path)),
        _ => None,
    }
}


fn return_ty_path(path: &TypePath) -> &Ident {
    path.path.get_ident().unwrap()
}
