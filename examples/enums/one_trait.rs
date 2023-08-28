use auto_delegate::{delegate, Delegate};

#[delegate]
trait Calc {
    fn calc(&self, x1: usize, x2: usize) -> usize;
}

#[derive(Default)]
struct CalcAdd;

impl Calc for CalcAdd {
    fn calc(&self, x1: usize, x2: usize) -> usize {
        x1 + x2
    }
}

#[derive(Default)]
struct CalcSub;

impl Calc for CalcSub {
    fn calc(&self, x1: usize, x2: usize) -> usize {
        x1 - x2
    }
}


#[derive(Delegate)]
#[to(Calc)]
enum EnumCalc {
    Add(CalcAdd),
    Sub(CalcSub),
}


fn main() {
    let c = EnumCalc::Add(CalcAdd::default());
    assert_eq!(c.calc(3, 5), 8);

    let c = EnumCalc::Sub(CalcSub::default());
    assert_eq!(c.calc(3, 2), 1);
}