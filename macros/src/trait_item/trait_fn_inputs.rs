use proc_macro2::Ident;
use syn::{FnArg, Pat, PatType, Receiver, TypeReference};
use syn::__private::TokenStream2;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::Type::{Path, Reference};

pub struct TraitFnInputs {
    inputs: Punctuated<FnArg, Comma>,
}


impl TraitFnInputs {
    pub const fn new(inputs: Punctuated<FnArg, Comma>) -> Self {
        Self { inputs }
    }


    pub fn expand_delegate_method(&self) -> Option<TokenStream2> {
        self.inputs
            .iter()
            .find_map(|args| match args {
                FnArg::Receiver(receiver) => expand_delegate_method(receiver),
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
                    Some(quote::quote! {#ident})
                }
                _ => None,
            })
            .intersperse(quote::quote! {,});

        quote::quote! {
            #(#expand)*
        }
    }

    pub fn expand_args(&self) -> syn::Result<TokenStream2> {
        let expand = self
            .inputs
            .iter()
            .map(|args| match args {
                FnArg::Receiver(receiver) => expand_receiver(receiver),
                //TODO: 引数の型の例外処理
                FnArg::Typed(pat_type) => expand_pat_type(pat_type).unwrap(),
            })
            .intersperse(quote::quote! {,});

        Ok(quote::quote! {
            #(#expand)*
        })
    }
}


fn expand_delegate_method(receiver: &Receiver) -> Option<TokenStream2> {
    let ty = *receiver.ty.clone();
    match ty {
        Reference(ty_ref) => Some(reference_delegate(&ty_ref)),
        _ => None,
    }
}


fn expand_receiver(receiver: &Receiver) -> TokenStream2 {
    let ty = *receiver.ty.clone();
    match ty {
        Reference(ty_ref) => reference_receiver(&ty_ref),
        Path(_) => quote::quote! {self},
        ty => syn::Error::new(ty.span(), "").to_compile_error(),
    }
}


fn expand_pat_type(pat_type: &PatType) -> syn::Result<TokenStream2> {
    let args_name = require_ident(&pat_type.pat)?;

    let args_ty = fn_args_ty(pat_type)?;

    Ok(quote::quote! {
        #args_name : #args_ty
    })
}


fn reference_delegate(ty_ref: &TypeReference) -> TokenStream2 {
    if ty_ref.mutability.is_none() {
        quote::quote! {self.delegate_by_ref()}
    } else {
        quote::quote! {self.delegate_by_mut()}
    }
}


fn reference_receiver(ty_ref: &TypeReference) -> TokenStream2 {
    let life_time = &ty_ref.lifetime;
    if ty_ref.mutability.is_none() {
        quote::quote! {&#life_time self}
    } else {
        quote::quote! {&#life_time mut self}
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


fn fn_args_ty(pat_type: &PatType) -> syn::Result<TokenStream2> {
    match *pat_type.clone().ty {
        Path(path) => {
            let ident = path
                .path
                .get_ident()
                .ok_or(syn::Error::new(
                    path.span(),
                    "Expected Input Ident But Not Found",
                ))?;
            Ok(quote::quote! {
                #ident
            })
        }
        _ => Err(syn::Error::new(pat_type.span(), "PatType is Not Supported")),
    }
}
