use auto_delegate::delegate;

#[delegate]
pub trait Calc {
    fn calc(&self, x1: usize, x2: usize) -> usize;
}

#[derive(Default)]
pub struct CalcAdd;


impl Calc for CalcAdd {
    fn calc(&self, x1: usize, x2: usize) -> usize {
        x1 + x2
    }
}

#[derive(Default)]
pub struct CalcSub;


impl Calc for CalcSub {
    fn calc(&self, x1: usize, x2: usize) -> usize {
        x1 - x2
    }
}

