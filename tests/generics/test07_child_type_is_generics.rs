use auto_delegate_impl::{delegate, Delegate};

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

#[derive(Delegate, Default)]
struct Parent<T: Calc + Default> {
    #[to(Calc)]
    child: T,
}

fn main() {
    let parent = Parent::<CalcAdd>::default();

    assert_eq!(parent.calc(3, 5), 8);

    let parent = Parent::<CalcSub>::default();
    assert_eq!(parent.calc(3, 2), 1);
}
