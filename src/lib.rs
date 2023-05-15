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


pub trait MacroMarkerRef<const S: char, const E: char> {
    type DelegateType;

    fn delegate_by_ref<'a, Output: 'a>(
        &'a self,
        f: impl FnOnce(&'a Self::DelegateType) -> Output,
    ) -> Output;
}


pub trait MacroMarkerMut<const S: char, const E: char> {
    type DelegateType;

    fn delegate_by_mut<'a, Output: 'a>(
        &'a mut self,
        f: impl FnOnce(&'a mut Self::DelegateType) -> Output,
    ) -> Output;
}
