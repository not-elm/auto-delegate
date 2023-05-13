#[cfg(test)]
#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/test1_delegate.rs");
    t.pass("tests/test2_nested_modules.rs");
    t.pass("tests/test3_delegate_with_args.rs");
    t.pass("tests/test4_delegate_tuple.rs");
    t.pass("tests/test5_delegate_reference.rs");
}
