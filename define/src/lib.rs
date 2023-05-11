pub trait MacroMarker {
    type DelegateType;

    fn delegate_by_ref(&self) -> &Self::DelegateType;

    fn delegate_by_mut(&mut self) -> &mut Self::DelegateType;
}

