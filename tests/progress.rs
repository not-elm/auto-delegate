#[cfg(test)]
#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01-impl_empty_trait.rs");
    t.pass("tests/02-derive_struct_empty.rs");
}
