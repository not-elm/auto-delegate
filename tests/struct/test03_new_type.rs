use auto_delegate_impl::{Delegate, delegate};

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
#[to(Calc)]
struct Parent(Add);

fn main() {
    let parent = Parent::default();
    assert_eq!(parent.calc(3, 2), 5);
}
