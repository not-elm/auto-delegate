use std::fmt::Debug;

use auto_delegate_impl::Delegate;

use crate::sub_module::multiple_field::child::MultipleChild;

#[derive(Delegate)]
pub struct MultipleParent<T>
where
    T: Debug + Default,
{
    #[to(Addr, Increment, Readable)]
    child: MultipleChild<T>,
}

impl<T> MultipleParent<T>
where
    T: Debug + Default,
{
    #[allow(unused)]
    pub fn new() -> MultipleParent<T> {
        Self {
            child: MultipleChild::default(),
        }
    }
}
