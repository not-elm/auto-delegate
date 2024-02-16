#![allow(unused)]


mod test01_lifetime;
mod test02_multiple_lifetimes;


#[cfg(test)]
#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/lifetime/test01_lifetime.rs");
    t.pass("tests/lifetime/test02_multiple_lifetimes.rs");
}

