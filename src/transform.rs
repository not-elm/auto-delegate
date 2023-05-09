use macros::delegate_trait;

#[delegate_trait]
pub trait Hello {
    fn hello(&mut self);
}


#[macro_export]
macro_rules! a {
    () => {};
}
