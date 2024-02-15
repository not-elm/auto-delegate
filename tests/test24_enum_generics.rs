use std::fmt::{Debug, Display};

use auto_delegate_impl::{delegate, Delegate};

#[delegate]
trait Calc {
    fn calc(&self, x1: usize, x2: usize) -> usize;
}


#[derive(Default)]
struct CalcAdd<T: Display>(T);

impl<T: Display> Calc for CalcAdd<T> {
    fn calc(&self, x1: usize, x2: usize) -> usize {
        x1 + x2
    }
}


struct CalcSub<'a, T, S, D>(&'a T, S, D) where T: Display;

impl<'a, T, S, D> Calc for crate::CalcSub<'a, T, S, D> where T: Display {
    fn calc(&self, x1: usize, x2: usize) -> usize {
        x1 - x2
    }
}


#[derive(Delegate)]
#[to(Calc)]
enum Parent<'a, T, S, D> where T: Debug + Display {
    Add(CalcAdd<T>),
    Sub(CalcSub<'a, T, S, D>),
}


fn main() {
    let parent = Parent::<'static, usize, String, String>::Add(CalcAdd::<usize>(0));
    assert_eq!(parent.calc(3, 2), 5);

    let num: usize = 0;
    let parent = Parent::<'_, usize, String, String>::Sub(CalcSub(&num, "".to_string(), "".to_string()));
    assert_eq!(parent.calc(3, 2), 1);
}
