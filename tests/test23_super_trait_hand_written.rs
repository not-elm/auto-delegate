use auto_delegate_impl::{delegate, Delegate};

trait Calc {
    fn calc(&self, x1: usize, x2: usize) -> usize;
}

#[delegate]
trait Label: Calc {
    fn label(&self) -> String;
}

#[derive(Default)]
struct CalcAdd;

impl Label for CalcAdd {
    fn label(&self) -> String {
        String::from("add")
    }
}

impl Calc for CalcAdd {
    fn calc(&self, x1: usize, x2: usize) -> usize {
        x1 + x2
    }
}

#[derive(Default, Delegate)]
#[to(Label)]
struct Parent(CalcAdd);

impl Calc for Parent {
    fn calc(&self, x1: usize, x2: usize) -> usize {
        x1 + x2
    }
}

fn main() {
    let parent = Parent::default();

    assert_eq!(parent.calc(3, 2), 5);
    assert_eq!(parent.label(), "add".to_string());
}
