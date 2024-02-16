#![allow(unused)]

mod test01_generics;
mod test02_multiple_generics;
mod test03_generics_bound;
mod test04_generics_where;
mod test05_trait_generics_bound;
mod test06_with_fn_generics;
mod test07_child_type_is_generics;

#[cfg(test)]
#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/generics/test01_generics.rs");
    t.pass("tests/generics/test02_multiple_generics.rs");
    t.pass("tests/generics/test03_generics_bound.rs");
    t.pass("tests/generics/test04_generics_where.rs");
    t.pass("tests/generics/test05_trait_generics_bound.rs");
    t.pass("tests/generics/test06_with_fn_generics.rs");
    t.pass("tests/generics/test07_child_type_is_generics.rs");
}

