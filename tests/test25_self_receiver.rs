use auto_delegate::{delegate, Delegate};

#[delegate]
trait Calc {
    fn calc(self, x1: usize, x2: usize) -> usize;
}


#[derive(Default)]
struct CalcAdd;

impl Calc for CalcAdd {
    fn calc(self, x1: usize, x2: usize) -> usize {
        x1 + x2
    }
}


#[derive(Default)]
struct CalcSub;

impl Calc for CalcSub {
    fn calc(self, x1: usize, x2: usize) -> usize {
        x1 - x2
    }
}


#[derive(Default)]
struct CalcMul;

impl Calc for CalcMul {
    fn calc(self, x1: usize, x2: usize) -> usize {
        x1 * x2
    }
}


#[derive(Default)]
struct CalcDiv;

impl Calc for CalcDiv {
    fn calc(self, x1: usize, x2: usize) -> usize {
        x1 / x2
    }
}


#[derive(Delegate)]
#[to(Calc)]
enum Calculator {
    Add(CalcAdd),
    Sub(CalcSub),
    Mul(CalcMul),
    Div(CalcDiv),
}


#[derive(Delegate)]
#[to(Calc)]
struct Parent<C: Calc>(C);

fn main() {
    // enum
    assert_eq!(Calculator::Add(CalcAdd).calc(3, 2), 5);
    assert_eq!(Calculator::Sub(CalcSub).calc(3, 2), 1);
    assert_eq!(Calculator::Mul(CalcMul).calc(3, 2), 6);
    assert_eq!(Calculator::Div(CalcDiv).calc(6, 2), 3);

    // struct
    assert_eq!(Parent(CalcAdd).calc(3, 2), 5);
    assert_eq!(Parent(CalcSub).calc(3, 2), 1);
    assert_eq!(Parent(CalcMul).calc(3, 2), 6);
    assert_eq!(Parent(CalcDiv).calc(6, 2), 3);
}
