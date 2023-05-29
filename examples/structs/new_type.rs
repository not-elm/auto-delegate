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


#[derive(Default, Delegate)]
#[to(Calc)]
struct Parent<T>(T) where T: Calc + Default;

fn main() {
    let parent = Parent::<CalcAdd>::default();
    assert_eq!(parent.calc(2, 3), 5);
}