use auto_delegate_macros::delegate;

pub mod parent;
pub mod child;

#[delegate]
pub trait Increment {
    fn increment(&mut self) -> usize;
}


