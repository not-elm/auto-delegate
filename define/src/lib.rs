pub trait MacroMarker {
    type DelegateType;

    fn delegate_by_ref(&self) -> &Self::DelegateType;

    fn delegate_by_mut(&mut self) -> &mut Self::DelegateType;
}


pub trait MacroDynMarker<T: ?Sized> {
    fn delegate_by_ref(&self) -> Box<&T>;

    fn delegate_by_mut(&mut self) -> Box<&mut T>;
}

