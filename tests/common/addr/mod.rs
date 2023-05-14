use auto_delegate_macros::delegate;

pub mod child;
pub mod parent;


#[delegate]
pub trait Addr {
    fn add(&self, x1: usize, x2: usize) -> usize;
}
