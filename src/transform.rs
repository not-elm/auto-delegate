use macros::{delegate_dyn_trait, delegate_trait};

#[delegate_trait]
pub trait Hello {
    fn hello(&mut self);
    fn hello_ref(&self, num: usize) -> usize;
}

#[delegate_dyn_trait]
pub trait DynHello {
    fn hello(&mut self);
    fn hello_ref(&self, num: usize) -> usize;
}
