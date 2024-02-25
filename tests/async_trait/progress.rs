#![allow(unused)]

mod test01_async_trait;
mod test02_async_trait_attribute_full_path;
mod test03_async_trait_attribute_short_path;

mod test04_async_trait_with_fn_lifetime;
mod test05_immutable_self_receiver;

#[cfg(test)]
#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/async_trait/test01_async_trait.rs");
    t.pass("tests/async_trait/test02_async_trait_attribute_full_path.rs");
    t.pass("tests/async_trait/test03_async_trait_attribute_short_path.rs");
    t.pass("tests/async_trait/test04_async_trait_with_fn_lifetime.rs");
    t.pass("tests/async_trait/test05_immutable_self_receiver.rs");
}

