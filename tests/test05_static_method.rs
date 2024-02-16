use auto_delegate_impl::{delegate, Delegate};

#[delegate]
trait StaticTrait {
    fn static_method1();

    fn static_method2() -> String;

    fn static_method3(str: &str, num: usize) -> &str;
}


struct Child;

impl StaticTrait for Child {
    fn static_method1() {

    }

    fn static_method2() -> String {
        "HELLO".to_string()
    }

    fn static_method3(str: &str, _num: usize) -> &str {
        str
    }
}


#[derive(Delegate)]
#[to(StaticTrait)]
struct Parent(Child);

fn main() {
    Parent::static_method1();
    assert_eq!(Parent::static_method2(), "HELLO");
    assert_eq!(Parent::static_method3("test", 0), "test");
}