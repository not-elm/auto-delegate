
use std::ops::Add;
use auto_delegate_impl::{delegate, Delegate};


#[delegate]
trait Calc{
    type Num: Add;

    fn calc(&self, x1: Self::Num, x2: Self::Num) -> Self::Num;
}

#[delegate]
trait NoBound{
    type Num;

    fn method();
}

struct Usize;

impl Calc for Usize {
    type Num = usize;

    fn calc(&self, x1: Self::Num, x2: Self::Num) -> Self::Num {
        x1 + x2
    }
}

struct Isize;

impl Calc for Isize {
    type Num = isize;

    fn calc(&self, x1: Self::Num, x2: Self::Num) -> Self::Num {
        x1 + x2
    }
}


#[derive(Delegate)]
struct Parent{
    #[to(Calc)]
    child: Usize
}

fn main(){
    let parent = Parent{
        child: Usize
    };
    assert_eq!(parent.calc(1, 2), 3);
}