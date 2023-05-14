use proc_macro2::Ident;

pub fn ident_prefix_and_suffix(ident: Ident) -> (char, char) {
    let str = ident_to_lower_string(ident);

    let chars: Vec<char> = str.chars().collect();
    let s = chars.first().unwrap();
    let e = chars.last().unwrap_or(s);
    (*s, *e)
}

pub fn ident_to_lower_string(ident: Ident) -> String {
    ident
        .to_string()
        .to_lowercase()
}


#[allow(unused)]
pub fn ident_from_str(s: &str) -> Ident {
    Ident::new(s, proc_macro2::Span::call_site())
}


#[allow(unused)]
pub fn generate_delegate_impl_macro_name(trait_ident: Ident) -> Ident {
    let macro_name = format!("impl_delegate_{}", ident_to_lower_string(trait_ident));
    Ident::new(&macro_name, proc_macro2::Span::call_site())
}
