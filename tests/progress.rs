#[cfg(test)]
#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/test1_delegate.rs");
    t.pass("tests/test2_nested_modules.rs");
    t.pass("tests/test3_delegate_with_args.rs");
    t.pass("tests/test4_delegate_tuple.rs");
    t.pass("tests/test5_delegate_reference.rs");
    t.pass("tests/test6_lifetime.rs");
    t.pass("tests/test7_multiple_lifetimes.rs");
    t.pass("tests/test8_generics.rs");
    t.pass("tests/test9_multiple_generics.rs");
    t.pass("tests/test10_type_slice.rs");
    t.pass("tests/test11_generics_bound.rs");
    t.pass("tests/test12_generics_where.rs");
    t.pass("tests/test13_multiple_delegate_traits.rs");
    t.pass("tests/test14_new_type.rs");
}
