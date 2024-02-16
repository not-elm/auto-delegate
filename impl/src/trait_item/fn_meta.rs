use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{FnArg, Pat, Receiver, TraitItemFn, Type, TypeReference};
use syn::__private::TokenStream2;
use syn::spanned::Spanned;
use syn::Type::Reference;

pub struct TraitFnMeta(TraitItemFn);


impl TraitFnMeta {
    pub const fn new(item_fn: TraitItemFn) -> Self {
        Self(item_fn)
    }

    pub fn expand_fn(&mut self, delegatable_ident: &TokenStream2) -> syn::Result<TokenStream2> {
        let stmt = self.delegate_stmt(delegatable_ident)?;

        self.0.default.replace(syn::parse2(quote!({
            #stmt
        })).unwrap());

        let f = &self.0;
        Ok(quote::quote! {
            #[inline(always)]
            #f
        })
    }


    fn delegate_stmt(&self, delegatable_ident: &TokenStream2) -> syn::Result<TokenStream2> {
        if let Some(receiver) = self.0.sig.receiver() {
            self.expand_delegate_receiver(receiver)
        } else {
            self.expand_delegate_static(delegatable_ident)
        }
    }


    fn expand_delegate_static(&self, delegatable_ident: &TokenStream2) -> syn::Result<TokenStream2> {
        let stmt = self.call_stmt();
        Ok(quote! {
            <Self as #delegatable_ident>::A::#stmt
        })
    }

    fn expand_delegate_receiver(&self, receiver: &Receiver) -> syn::Result<TokenStream2> {
        let ty = *receiver.ty.clone();
        match ty {
            Type::Path(_) => Ok(self.delegate_owned()),
            Reference(ty_ref) => Ok(self.delegate_ref_or_mut(&ty_ref)),
            _ => Err(
                syn::Error::new(
                    Span::call_site(),
                    format!("expect type must be one of 'self', '&self', '&mut self' bad was {ty:?}"),
                )),
        }
    }

    fn delegate_owned(&self) -> TokenStream2 {
        let call = self.call();
        quote::quote! {
            let m = self.delegate_by_owned();
            #call
        }
    }


    fn delegate_ref_or_mut(&self, ty_ref: &TypeReference) -> TokenStream2 {
        let call = self.call();

        if ty_ref.mutability.is_none() {
            quote::quote! {
                let m = self.delegate_by_ref();
                #call
            }
        } else {
            quote::quote! {
                let m = self.delegate_by_mut();
                #call
            }
        }
    }


    fn call_stmt(&self) -> TokenStream2 {
        let inputs = self.expand_inputs();
        let fn_name = &self.0.sig.ident;
        let asyncness = self.0.sig.asyncness.map(|_|quote!(.await));
        if self.0.sig.generics.params.is_empty() {
            quote!(#fn_name(#inputs)#asyncness)
        } else {
            let (_, generics, _) = &self.0.sig.generics.split_for_impl();
            quote!(#fn_name::#generics(#inputs)#asyncness)
        }
    }

    fn call(&self) -> TokenStream2 {
        let stmt = self.call_stmt();
        let call = quote!(return t.#stmt;);
        quote! {
            if let Some(t) = m.0{
                #call
            }
            if let Some(t) = m.1{
                #call
            }
            if let Some(t) = m.2{
                #call
            }
            if let Some(t) = m.3{
                #call
            }
            if let Some(t) = m.4{
                #call
            }
            if let Some(t) = m.5{
                #call
            }
            if let Some(t) = m.6{
                #call
            }
            if let Some(t) = m.7{
                #call
            }
            if let Some(t) = m.8{
                #call
            }
            if let Some(t) = m.9{
                #call
            }
            if let Some(t) = m.10{
                #call
            }
            if let Some(t) = m.11{
                #call
            }
            panic!("unreachable");
        }
    }


    fn expand_inputs(&self) -> TokenStream2 {
        let inputs = self
            .0
            .sig
            .inputs
            .iter()
            .filter_map(|args| match args {
                FnArg::Typed(pat_type) => {
                    let ident = require_ident(&pat_type.pat).ok()?;
                    Some(quote::quote! {#ident,})
                }
                _ => None,
            });

        quote::quote! {
            #(#inputs)*
        }
    }
}

fn require_ident(pat: &Pat) -> syn::Result<&Ident> {
    if let Pat::Ident(ident) = pat {
        Ok(&ident.ident)
    } else {
        Err(syn::Error::new(
            pat.span(),
            "Required PatIdent But Different",
        ))
    }
}
