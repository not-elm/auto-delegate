#[cfg(test)]
#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01-generate_impl_macro_empty_trait.rs");
    t.pass("tests/02-generate_impl_macro_with_method.rs");
    t.pass("tests/03-generate_impl_macro_with_input.rs");
    // t.pass("tests/03-generate_impl_macro_with_input");
    // t.pass("tests/05-impl_by_delegate.rs");
}
