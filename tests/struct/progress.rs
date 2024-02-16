#![allow(unused)]

mod test01_delegate;
mod test02_multiple_fields;
mod test03_new_type;
mod test04_unnamed_fields;

#[cfg(test)]
#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/struct/test01_delegate.rs");
    t.pass("tests/struct/test02_multiple_fields.rs");
    t.pass("tests/struct/test03_new_type.rs");
    t.pass("tests/struct/test04_unnamed_fields.rs");
}

