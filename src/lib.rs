use std::marker::PhantomData;

pub trait MacroMarker {
    type DelegateType;

    fn delegate_by_ref(&self) -> &Self::DelegateType;

    fn delegate_by_mut(&mut self) -> &mut Self::DelegateType;
}


pub trait MacroDynMarker<T> {
    fn delegate_by_ref(&self) -> &T;

    fn delegate_by_mut(&mut self) -> &mut T;
}



struct A;

trait Hello{
    fn a(&self );
}


struct B;
impl Hello for B{

}

impl<T: MacroDynMarker<U>, U: Hello> Hello for (T, PhantomData<U>){
    fn a(&self){
        self.0.delegate_by_ref();

    }
}


impl MacroDynMarker<B> for A {
    fn delegate_by_ref(&self) -> &B {
            todo!()
    }

    fn delegate_by_mut(&mut self) -> &mut B{
        todo!()
    }
}


fn a(){
    let parent = A{};
    (parent, &B{}).
}