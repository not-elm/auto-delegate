use auto_delegate_impl::{delegate, Delegate};

#[delegate]
trait Calc {
    fn calc(&self, x1: usize, x2: usize) -> usize;
}

#[delegate]
trait Label {
    fn label(&self) -> String;
}

#[delegate]
trait Increment: Calc + Label {
    fn increment(&mut self) -> usize;
}

#[derive(Default)]
struct Child {
    num: usize,
}

impl Increment for Child {
    fn increment(&mut self) -> usize {
        self.num += 1;
        self.num
    }
}

impl Calc for Child {
    fn calc(&self, x1: usize, x2: usize) -> usize {
        x1 + x2
    }
}

impl Label for Child {
    fn label(&self) -> String {
        String::from("add")
    }
}

#[derive(Default, Delegate)]
struct Parent {
    #[to(Calc, Increment, Label)]
    child: Child,
}

fn main() {
    let mut parent = Parent::default();
    assert_eq!(parent.increment(), 1);
    assert_eq!(parent.increment(), 2);
    assert_eq!(parent.calc(3, 2), 5);
    assert_eq!(parent.label(), "add");
}
