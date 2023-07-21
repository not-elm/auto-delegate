use std::fmt::Debug;

use auto_delegate_impl::{delegate, Delegate};

#[delegate]
trait Calc {
    fn calc(&self, x1: usize, x2: usize) -> usize;
}


#[derive(Default)]
struct CalcAdd<T>(T);

impl<T> Calc for CalcAdd<T> {
    fn calc(&self, x1: usize, x2: usize) -> usize {
        x1 + x2
    }
}


#[derive(Delegate)]
#[to(Calc)]
enum Parent<T> where T: Debug {
    Add(CalcAdd<T>)
}


fn main() {
    let parent = Parent::Add(CalcAdd::<usize>(0));

    assert_eq!(parent.calc(3, 2), 5);
}
