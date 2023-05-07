use proc_macro::TokenStream;

use quote::TokenStreamExt;
use syn::ItemStruct;
use syn::__private::TokenStream2;

use crate::delegate_struct::by_fields::ByFields;
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

    let by_field = ByFields::new(item_struct.fields.clone())
        .next()
        .unwrap();
    let delegate_field_name = by_field.field_name_ref();

    let struct_name = item_struct.ident;

    let mut expand = quote::quote!();
    by_field
        .trait_names_ref()
        .iter()
        .for_each(|trait_name| {
            let macro_name = generate_delegate_impl_macro_name(trait_name.clone());

            let impl_delegate_field = quote::quote! {
                #macro_name!(#struct_name, #delegate_field_name);
            };
            expand.extend(impl_delegate_field)
        });
    Ok(expand)
}
