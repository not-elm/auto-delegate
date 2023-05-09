use proc_macro::TokenStream;

use proc_macro2::Ident;
use syn::ItemStruct;
use syn::__private::TokenStream2;
use syn::spanned::Spanned;

use crate::delegate_struct::by_fields::{ByField, ByFields};
use crate::ident::generate_delegate_impl_macro_name;

mod by_fields;

pub fn expand_delegate(input: TokenStream) -> proc_macro2::TokenStream {
    match try_expand_delegate(input) {
        Ok(output) => output,
        Err(error) => error.to_compile_error(),
    }
}

fn try_expand_delegate(input: TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    let item_struct = syn::parse::<ItemStruct>(input)?;
    let struct_name = item_struct.clone().ident;
    let crate_path = crate_path(item_struct.span().unwrap());

    let expand_impl_methods = ByFields::new(item_struct.fields)
        .map(|by_field| impl_method_by_delegate(&crate_path, &struct_name, by_field));


    Ok(quote::quote! {
        #(#expand_impl_methods)*
    })
}


fn impl_method_by_delegate(
    crate_path: &Option<TokenStream2>,
    struct_name: &Ident,
    by_field: ByField,
) -> proc_macro2::TokenStream {
    let delegate_field_name = by_field.field_name_ref();


    let mut expand = quote::quote!();
    by_field
        .trait_names_ref()
        .iter()
        .for_each(|trait_name| {
            let macro_name = generate_delegate_impl_macro_name(trait_name.clone());

            let impl_delegate_field = quote::quote! {
                use crate::#macro_name;

                #macro_name!(#struct_name, #delegate_field_name);
            };
            expand.extend(impl_delegate_field)
        });


    expand
}


fn crate_path(span: proc_macro::Span) -> Option<TokenStream2> {
    if span
        .source_file()
        .path()
        .iter()
        .any(|p| p == "tests")
    {
        None
    } else {
        Some(quote::quote! {$crate::})
    }
}
