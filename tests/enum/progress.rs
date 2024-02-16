#![allow(unused)]

mod test01_enum;
mod test02_multiple_traits;
mod test03_super_trait;
mod test04_generic;
mod test05_many_variants;

#[cfg(test)]
#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/enum/test01_enum.rs");
    t.pass("tests/enum/test02_multiple_traits.rs");
    t.pass("tests/enum/test03_super_trait.rs");
    t.pass("tests/enum/test04_generic.rs");
    t.pass("tests/enum/test05_many_variants.rs");
}

