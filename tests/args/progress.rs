#![allow(unused)]

mod test01_method_with_args;
mod test02_tuple;
mod test03_reference;

fn main() {
    let t = trybuild::TestCases::new();
    t.pass("tests/args/test01_method_with_args.rs");
    t.pass("tests/args/test02_tuple.rs");
    t.pass("tests/args/test03_reference.rs");
    t.pass("tests/args/test04_slice.rs");
}