use auto_delegate::{delegate, Delegate};

#[delegate]
trait Calc {
    fn calc(&self, x1: usize, x2: usize) -> usize;
}


#[delegate]
trait Label: Calc {
    fn label(&self) -> String;
}


#[derive(Default)]
struct CalcAdd;


impl Label for CalcAdd {
    fn label(&self) -> String {
        String::from("add")
    }
}


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


impl Label for CalcSub {
    fn label(&self) -> String {
        String::from("sub")
    }
}


#[derive(Delegate)]
#[to(Calc, Label)]
enum Calculator {
    Add(CalcAdd),
    Sub(CalcSub),
}


fn main() {
    let add = Calculator::Add(CalcAdd);
    
    assert_eq!(add.calc(3, 2), 5);
    assert_eq!(add.label(), "add".to_string());

    let sub = Calculator::Sub(CalcSub);

    assert_eq!(sub.calc(3, 2), 1);
    assert_eq!(sub.label(), "sub".to_string());
}
