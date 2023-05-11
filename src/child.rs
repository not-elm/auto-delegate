use crate::transform::Hello;

pub struct HelloChild {}

impl Hello for HelloChild {
    fn hello(&mut self) {}

    fn hello_ref(&self,num:usize) -> usize {
        num + 1
    }
}
