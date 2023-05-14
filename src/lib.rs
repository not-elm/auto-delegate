pub trait MacroMarker<const S: char, const E: char> {
    type DelegateType;

    fn delegate_by_ref(&self) -> &Self::DelegateType;

    fn delegate_by_mut(&mut self) -> &mut Self::DelegateType;
}


pub trait MacroDynMarker<const H: char> {
    type Delegate;
    fn delegate_by_ref<F>(&self, fun: F) where F: FnOnce(&F) -> &Self::Delegate;

    fn delegate_by_mut(&mut self) -> &mut Self::Delegate;
}

