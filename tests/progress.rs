#[cfg(test)]
#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/test01_nested_modules.rs");
    t.pass("tests/test02_super_trait.rs");
    t.pass("tests/test03_super_trait_hand_written.rs");
    t.pass("tests/test04_self_receiver.rs");
    t.pass("tests/test05_static_method.rs");
}

