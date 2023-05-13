use auto_delegate_macros::delegate_trait;

pub mod parent;
pub mod child;

#[delegate_trait]
pub trait Addr {
    fn increment(&mut self) -> usize;
}


