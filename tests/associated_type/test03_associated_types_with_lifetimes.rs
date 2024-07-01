
use std::ops::Add;
use auto_delegate_impl::{delegate, Delegate};


#[delegate]
trait Calc<'a, Num: Add>{
    type Out;

    fn calc(&self, x1: &'a Num, x2: &'a Num) -> Self::Out;
}

struct Usize;
impl<'a> Calc<'a, usize> for Usize{
    type Out = u8;

    fn calc(&self, x1: &'a usize, x2: &'a usize) -> Self::Out {
        (x1 + x2) as u8
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
    assert_eq!(parent.calc(&1, &2), 3);
}