use proc_macro2::Ident;
use quote::quote;
use syn::{FnArg, Pat, Receiver, Type, TypeReference};
use syn::__private::TokenStream2;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::Type::Reference;

pub struct TraitFnInputs {
    inputs: Punctuated<FnArg, Comma>,
}


impl TraitFnInputs {
    pub const fn new(inputs: Punctuated<FnArg, Comma>) -> Self {
        Self { inputs }
    }


    pub fn expand_delegate_method(&self, fn_name: &Ident) -> Option<TokenStream2> {
        self.inputs
            .iter()
            .find_map(|args| match args {
                FnArg::Receiver(receiver) => self.expand_delegate_receiver(receiver, fn_name),
                _ => None,
            })
    }


    pub fn expand_inputs(&self) -> TokenStream2 {
        let expand = self
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
            #(#expand)*
        }
    }


    fn expand_delegate_receiver(&self, receiver: &Receiver, fn_name: &Ident) -> Option<TokenStream2> {
        let ty = *receiver.ty.clone();
        match ty {
            Type::Path(_) => Some(self.receiver(fn_name)),
            Reference(ty_ref) => Some(self.reference_delegate(&ty_ref, fn_name)),
            _ => None,
        }
    }


    fn receiver(&self, fn_name: &Ident) -> TokenStream2 {
        let call = self.call(fn_name);
        quote::quote! {
            let m = self.delegate_by_owned();
            #call
        }
    }


    fn reference_delegate(&self, ty_ref: &TypeReference, fn_name: &Ident) -> TokenStream2 {
        let call = self.call(fn_name);

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

    fn call(&self, fn_name: &Ident) -> TokenStream2 {
        let inputs = self.expand_inputs();

        quote! {
            if let Some(t) = m.0{
                return t.#fn_name(#inputs);
            }
            if let Some(t) = m.1{
                return t.#fn_name(#inputs);
            }
            if let Some(t) = m.2{
                return t.#fn_name(#inputs);
            }
            if let Some(t) = m.3{
                return t.#fn_name(#inputs);
            }
            if let Some(t) = m.4{
                return t.#fn_name(#inputs);
            }
            if let Some(t) = m.5{
                return t.#fn_name(#inputs);
            }
            if let Some(t) = m.6{
                return t.#fn_name(#inputs);
            }
            if let Some(t) = m.7{
                return t.#fn_name(#inputs);
            }
            if let Some(t) = m.8{
                return t.#fn_name(#inputs);
            }
            if let Some(t) = m.9{
                return t.#fn_name(#inputs);
            }
            if let Some(t) = m.10{
                return t.#fn_name(#inputs);
            }
            if let Some(t) = m.11{
                return t.#fn_name(#inputs);
            }
            panic!("unreachable");
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
