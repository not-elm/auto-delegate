use auto_delegate_impl::delegate;

pub mod child;
pub mod parent;

#[delegate]
pub trait Increment {
    fn increment(&mut self) -> usize;
}
