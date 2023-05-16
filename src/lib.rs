pub use auto_delegate_macros::{
    Delegate,
    delegate,
};


pub trait MacroMarker<const S: char, const E: char> {
    type DelegateType: ?Sized;

    fn delegate_by_ref<'a, Output: 'a>(
        &'a self,
        f: impl FnOnce(&'a Self::DelegateType) -> Output,
    ) -> Output;

    fn delegate_by_mut<'a, Output: 'a>(
        &'a mut self,
        f: impl FnOnce(&'a mut Self::DelegateType) -> Output,
    ) -> Output;
}

