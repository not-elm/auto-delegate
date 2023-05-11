use macros::delegate_trait;

#[delegate_trait]
pub trait Hello {
    fn hello(&mut self);
    fn hello_ref(&self, num: usize) -> usize;
}

