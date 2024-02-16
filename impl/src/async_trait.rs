use syn::parse::{Parse, ParseStream};
use syn::Token;

#[derive(Copy, Clone)]
pub struct AsyncTraitArgs {
    pub local: bool,
}

mod kw {
    syn::custom_keyword!(Send);
}

impl Parse for AsyncTraitArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Token![?]) {
            input.parse::<Token![?]>()?;
            input.parse::<kw::Send>()?;
            Ok(AsyncTraitArgs { local: true })
        } else {
            Ok(AsyncTraitArgs { local: false })
        }
    }
}


