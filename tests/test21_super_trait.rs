use auto_delegate_macros::{delegate, Delegate};

#[delegate]
trait Calc {
    fn calc(&self, x1: usize, x2: usize) -> usize;
}


#[delegate]
trait Increment: Calc {
    fn increment(&mut self) -> usize;
}


#[derive(Default)]
struct Child {
    num: usize,
}


impl Increment for Child {
    fn increment(&mut self) -> usize {
        self.num += 1;
        self.num
    }
}


impl Calc for Child {
    fn calc(&self, x1: usize, x2: usize) -> usize {
        x1 + x2
    }
}


#[derive(Default, Delegate)]
struct Parent {
    #[to(Calc, Increment)]
    child: Child,
}


fn main() {
    let mut parent = Parent::default();
    assert_eq!(parent.increment(), 1);
    assert_eq!(parent.increment(), 2);
    assert_eq!(parent.calc(3, 2), 5);
}
