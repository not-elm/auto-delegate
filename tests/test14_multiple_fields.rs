use auto_delegate_impl::{delegate, Delegate};

use crate::common::addr::Addr;
use crate::common::increment::Increment;
use crate::common::multiple_field::child::MultipleChild;

mod common;

#[delegate]
trait Hello {
    fn hello(&mut self);
}

#[delegate]
trait Number {
    fn num(&self) -> usize;
}

#[derive(Default)]
struct HelloChild;

impl Hello for HelloChild {
    fn hello(&mut self) {}
}

impl Number for HelloChild {
    fn num(&self) -> usize {
        3
    }
}

#[derive(Delegate, Default)]
struct Parent {
    #[to(Addr, Increment)]
    child: MultipleChild<usize>,

    #[to(Hello, Number)]
    child2: HelloChild,
}

fn main() {
    let mut parent = Parent::default();

    assert_eq!(parent.add(2, 3), 5);

    assert_eq!(parent.increment(), 1);

    parent.hello();

    assert_eq!(parent.num(), 3);
}
