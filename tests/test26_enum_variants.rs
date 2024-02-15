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
    Add2(CalcAdd),
    Sub2(CalcSub),
    Mul2(CalcMul),
    Div2(CalcDiv),
    Add3(CalcAdd),
    Sub3(CalcSub),
    Mul3(CalcMul),
    Div3(CalcDiv),
}


fn main() {
    assert_eq!(Calculator::Add(CalcAdd).calc(3, 2), 5);
    assert_eq!(Calculator::Sub(CalcSub).calc(3, 2), 1);
    assert_eq!(Calculator::Mul(CalcMul).calc(3, 2), 6);
    assert_eq!(Calculator::Div(CalcDiv).calc(6, 2), 3);

    assert_eq!(Calculator::Add2(CalcAdd).calc(3, 2), 5);
    assert_eq!(Calculator::Sub2(CalcSub).calc(3, 2), 1);
    assert_eq!(Calculator::Mul2(CalcMul).calc(3, 2), 6);
    assert_eq!(Calculator::Div2(CalcDiv).calc(6, 2), 3);

    assert_eq!(Calculator::Add3(CalcAdd).calc(3, 2), 5);
    assert_eq!(Calculator::Sub3(CalcSub).calc(3, 2), 1);
    assert_eq!(Calculator::Mul3(CalcMul).calc(3, 2), 6);
    assert_eq!(Calculator::Div3(CalcDiv).calc(6, 2), 3);
}
