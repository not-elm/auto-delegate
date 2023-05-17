use auto_delegate::{delegate, Delegate};

#[delegate]
trait Calc {
    fn calc(&mut self, x1: usize, x2: usize) -> &usize;
}

struct CalcAdd {
    num: usize,
}


impl Calc for CalcAdd {
    fn calc(&mut self, x1: usize, x2: usize) -> &usize {
        self.num = x1 + x2;
        &self.num
    }
}

struct CalcSub {
    num: usize,
}

impl Calc for CalcSub {
    fn calc(&mut self, x1: usize, x2: usize) -> &usize {
        self.num = x1 - x2;
        &self.num
    }
}


#[derive(Delegate)]
#[to(Calc)]
enum EnumCalc {
    Add(CalcAdd),
    Sub(CalcSub),
}


fn main() {
    let mut c = EnumCalc::Add(CalcAdd { num: 0 });
    assert_eq!(*c.calc(3, 5), 8);


    let mut c = EnumCalc::Sub(CalcSub { num: 0 });
    assert_eq!(*c.calc(3, 2), 1);
}
