use std::fmt::Debug;

use auto_delegate_impl::delegate;

use crate::sub_module::addr::Addr;
use crate::sub_module::increment::Increment;

#[delegate]
pub trait Readable<T>
where
    T: Debug + Default,
{
    fn read(&self) -> &T;
}

pub struct MultipleChild<T> {
    num: usize,
    value: T,
}

impl<T: Debug + Default> Default for MultipleChild<T> {
    fn default() -> Self {
        Self {
            num: 0,
            value: T::default(),
        }
    }
}

impl<T> Addr for MultipleChild<T>
where
    T: Debug + Default,
{
    fn add(&self, x1: usize, x2: usize) -> usize {
        x1 + x2
    }
}

impl<T> Increment for MultipleChild<T>
where
    T: Debug + Default,
{
    fn increment(&mut self) -> usize {
        self.num += 1;
        self.num
    }
}

impl<T> Readable<T> for MultipleChild<T>
where
    T: Debug + Default,
{
    fn read(&self) -> &T {
        &self.value
    }
}
