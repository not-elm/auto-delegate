use syn::Type;
use syn::__private::TokenStream2;

pub fn expand_syn_type(ty: &Type) -> TokenStream2 {
    quote::quote!(#ty)
}


