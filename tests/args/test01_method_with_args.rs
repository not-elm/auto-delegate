use auto_delegate_impl::{delegate, Delegate};

#[delegate]
trait Calc {
    fn calc(&self, lhs: usize, rhs: usize) -> usize;
}


#[derive(Default)]
struct Add;

impl Calc for Add {
    fn calc(&self, lhs: usize, rhs: usize) -> usize {
        lhs + rhs
    }
}


#[derive(Delegate, Default)]
struct Calculator {
    #[to(Calc)]
    base: Add,
}

fn main() {
    let calc = Calculator::default();
    assert_eq!(calc.calc(1, 2), 3);
}