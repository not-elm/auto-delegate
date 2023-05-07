#[cfg(test)]
#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01_generate_impl_macro_empty_trait.rs");
    t.pass("tests/02_delegate_by_nothing_args.rs");
    t.pass("tests/03_delegate_by_with_args.rs");
    t.pass("tests/04_delegate_multiple_methods.rs");
    t.pass("tests/05_delegate_multiple_traits.rs");
    t.pass("tests/06_delegate_multiple_fields.rs");
    t.pass("tests/07_output_tuple.rs");
}
