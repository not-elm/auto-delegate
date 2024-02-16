use std::fmt::Display;
use auto_delegate_impl::{delegate, Delegate};

#[delegate]
trait Trait<S: Display>{
    fn s(&self) -> S;
}


#[delegate]
trait Trait2<'a, 'b, 'c, S> where S: Display{
    fn s2(&self) -> S;
}

struct Child;

impl Trait<String> for Child{
    fn s(&self) -> String {
        "test".to_string()
    }
}


impl<'c> Trait2<'static, 'static, 'c, String> for Child {
    fn s2(&self) -> String {
        "todo!()".to_string()
    }
}

#[derive(Delegate)]
#[to(Trait, Trait2)]
struct Parent(Child);

fn main(){
    let p = Parent(Child);
    p.s();
    p.s2();
}