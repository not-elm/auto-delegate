use proc_macro2::Ident;
use syn::__private::TokenStream2;
use syn::{ReturnType, TraitItemFn, Type, TypePath, TypeReference, TypeTuple};

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
            return_ty(type_path).map(|ty| quote::quote! {-> #ty})
        } else {
            None
        }
    }
}


fn return_ty(ty: &Type) -> Option<TokenStream2> {
    match ty {
        Type::Path(path) => Some(output_path(path)),
        Type::Tuple(tuple) => output_tuple(tuple),
        Type::Reference(reference) => Some(output_ref(reference)),
        _ => None,
    }
}


fn output_path(path: &TypePath) -> TokenStream2 {
    let ty = return_ty_path(path);
    quote::quote! {#ty}
}


fn output_tuple(tuple: &TypeTuple) -> Option<TokenStream2> {
    let first = tuple.elems.first()?;
    let last = tuple.elems.last()?;
    Some(quote::quote! {(#first, #last)})
}


fn output_ref(reference: &TypeReference) -> TokenStream2 {
    let elem = &reference.elem;
    let life_time = &reference.lifetime;
    if reference.mutability.is_some() {
        quote::quote! {&#life_time mut #elem}
    } else {
        quote::quote! {&#life_time #elem}
    }
}


fn return_ty_path(path: &TypePath) -> &Ident {
    path.path.get_ident().unwrap()
}
