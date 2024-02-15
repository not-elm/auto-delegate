#![allow(late_bound_lifetime_arguments)]

use std::marker::PhantomData;

use auto_delegate_impl::{delegate, Delegate};

#[delegate]
trait Increment {
    fn increment(&mut self) -> usize;
}


#[delegate]
trait Add {
    fn add(&self, lhs: usize, rhs: usize) -> usize;
}


#[derive(Default)]
struct Child(usize);

impl Increment for Child {
    fn increment(&mut self) -> usize {
        self.0 += 1;
        self.0
    }
}

impl Add for Child {
    fn add(&self, lhs: usize, rhs: usize) -> usize {
        lhs + rhs
    }
}

#[derive(Delegate, Default)]
struct Parent(#[to(Increment)]Child, PhantomData<Child>, #[to(Add)] Child);

fn main() {
    let mut p = Parent::default();
    assert_eq!(p.increment(), 1);
    assert_eq!(p.increment(), 2);
    assert_eq!(p.increment(), 3);

    assert_eq!(p.add(1, 2), 3);
}