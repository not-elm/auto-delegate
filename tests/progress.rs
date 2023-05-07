#[cfg(test)]
#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01-generate_impl_macro_empty_trait.rs");
    t.pass("tests/02-delegate_by_nothing_args.rs");
    t.pass("tests/03-delegate_by_with_args.rs");
    t.pass("tests/04-delegate_multiple_methods.rs");
    t.pass("tests/05-delegate_multiple_traits.rs");
}
