use auto_delegate_macros::delegate_trait;

pub mod child;
pub mod parent;


#[delegate_trait]
pub trait Addr {
    fn add(
        &self,
        x1: usize,
        x2: usize) -> usize;
}