use proc_macro2::Ident;
use syn::{FnArg, Pat, PatType, Receiver, TypeReference};
use syn::__private::TokenStream2;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::Type::{Path, Reference};

use crate::syn::syn_type::expand_syn_type;

pub struct TraitFnInputs {
    inputs: Punctuated<FnArg, Comma>,
}


impl TraitFnInputs {
    pub const fn new(inputs: Punctuated<FnArg, Comma>) -> Self {
        Self { inputs }
    }


    pub fn expand_delegate_method(&self, fn_name: &Ident, trait_name: &TokenStream2) -> Option<TokenStream2> {
        self.inputs
            .iter()
            .find_map(|args| match args {
                FnArg::Receiver(receiver) => self.expand_delegate_receiver(receiver, fn_name, trait_name),
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
        let mut expand: Vec<TokenStream2> = Vec::new();
        for args_type in self.inputs.iter() {
            let token = self.expand_fn_arg(args_type)?;
            expand.push(token);
        }

        let expand = expand
            .into_iter()
            .intersperse(quote::quote!(,));

        Ok(quote::quote! {
            #(#expand)*
        })
    }


    fn expand_delegate_receiver(&self, receiver: &Receiver, fn_name: &Ident, trait_name: &TokenStream2) -> Option<TokenStream2> {
        let ty = *receiver.ty.clone();
        match ty {
            Reference(ty_ref) => Some(self.reference_delegate(&ty_ref, fn_name, trait_name)),
            _ => None,
        }
    }


    fn expand_receiver(&self, receiver: &Receiver) -> TokenStream2 {
        let ty = *receiver.ty.clone();

        match ty {
            Reference(ty_ref) => self.reference_receiver(&ty_ref),
            Path(_) => quote::quote! {self},
            ty => syn::Error::new(ty.span(), format!("This Receiver Type Not Supported = ({})", quote::quote!(#ty).to_string())).to_compile_error(),
        }
    }

    fn reference_receiver(&self, ty_ref: &TypeReference) -> TokenStream2 {
        let life_time = &ty_ref.lifetime;
        if ty_ref.mutability.is_none() {
            quote::quote! {&#life_time self}
        } else {
            quote::quote! {&#life_time mut self}
        }
    }


    fn reference_delegate(&self, ty_ref: &TypeReference, fn_name: &Ident, trait_name: &TokenStream2) -> TokenStream2 {
        let inputs = self.expand_inputs();


        if ty_ref.mutability.is_none() {
            quote::quote! {self.delegate_by_ref(|f: &#trait_name|{
                f.#fn_name(#inputs)
            })}
        } else {
            quote::quote! {self.delegate_by_mut(|f: &mut #trait_name|{
                 f.#fn_name(#inputs)
            })}
        }
    }


    fn expand_fn_arg(&self, args: &FnArg) -> syn::Result<TokenStream2> {
        match args {
            FnArg::Receiver(receiver) => Ok(self.expand_receiver(receiver)),

            FnArg::Typed(pat_type) => expand_pat_type(pat_type),
        }
    }
}


fn expand_pat_type(pat_type: &PatType) -> syn::Result<TokenStream2> {
    let args_name = require_ident(&pat_type.pat)?;


    let args_ty = expand_syn_type(&pat_type.ty);

    Ok(quote::quote! {
        #args_name : #args_ty
    })
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
