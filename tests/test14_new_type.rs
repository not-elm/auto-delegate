#![feature(impl_trait_in_assoc_type)]

use std::ops::Add;
use auto_delegate::MacroMarker;
use auto_delegate_macros::delegate;

use crate::common::increment::Increment;

mod common;

#[delegate]
trait Calc {
    fn calc(&self, x1: usize, x2: usize) -> usize;
}

struct Addr;

impl Calc for Addr {
    fn calc(&self, x1: usize, x2: usize) -> usize {
        x1 + x2
    }
}

struct Subtract;


impl Calc for Subtract {
    fn calc(&self, x1: usize, x2: usize) -> usize {
        x1 - x2
    }
}


enum Parent {
    Add(Addr),
    Sub(Subtract),
}


fn main() {}
