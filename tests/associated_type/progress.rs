#![allow(unused)]

mod test01_associated_type;
mod test02_associated_types_with_generics;
mod test03_associated_types_with_lifetimes;

#[cfg(test)]
#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/associated_type/test01_associated_type.rs");
    t.pass("tests/associated_type/test02_associated_types_with_generics.rs");
    t.pass("tests/associated_type/test03_associated_types_with_lifetimes.rs");
}