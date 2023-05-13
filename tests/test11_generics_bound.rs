use std::{marker::PhantomData, ops::Add};

use auto_delegate_macros::{Delegate, delegate_trait};

#[delegate_trait]
trait Addr<Num: Add<Output = Num>> {
    fn add(&mut self, x1: Num, x2: Num) -> Num;
}


struct Child<Num: Add<Output = Num>> {
    _maker: PhantomData<Num>,
}


impl<Num: Add<Output = Num>> Addr<Num> for Child<Num> {
    fn add(&mut self, x1: Num, x2: Num) -> Num {
        x1 + x2
    }
}


#[derive(Delegate)]
struct Parent<Num: Add<Output = Num>> {
    #[by(Addr)]
    child: Child<Num>,
}


impl<Num: Add<Output = Num>> Parent<Num> {
    pub fn new() -> Parent<Num> {
        Self {
            child: Child {
                _maker: PhantomData,
            },
        }
    }
}


fn main() {
    let mut parent = Parent::<usize>::new();
    let num = parent.add(3, 5);

    assert_eq!(num, 8);
}
