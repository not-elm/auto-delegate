#![feature(impl_trait_in_assoc_type)]

use std::ops::Deref;
use std::rc::Rc;
use auto_delegate::MacroMarker;
use auto_delegate_macros::delegate;

#[delegate]
trait Calc {
    fn calc(&mut self, x1: usize, x2: usize) -> &usize;
}

struct C1{
    num: usize
}
impl C1 {
    fn hello(
        &mut self,
        f: impl FnOnce(&mut dyn Calc) -> &usize) -> &usize {
        f(self)
    }
}

impl Calc for C1 {
    fn calc(&mut self, x1: usize, x2: usize) -> &usize {
        self.num = x1 + x2;
        &self.num
    }
}

struct C2;

impl Calc for C2 {
    fn calc(&mut self, x1: usize, x2: usize) -> &usize {
        todo!()
    }
}

enum C {
    C1(C1),
    C2(C2),
}


impl MacroMarker<'c', 'c'> for C {
    type DelegateType = dyn Calc;

    fn delegate_by_ref<'a, Output: 'a>(&'a self, f: impl FnOnce(&'a Self::DelegateType) -> Output) -> Output {
        match self {
            Self::C1(c1) => f(c1),
            Self::C2(c2) => f(c2)
        }
    }

    fn delegate_by_mut<'a, Output: 'a>(&'a mut self, f: impl FnOnce(&'a mut Self::DelegateType) -> Output) -> Output {
        todo!()
    }
}



