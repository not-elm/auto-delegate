use crate::transform::Hello;

pub struct HelloChild {}

impl Hello for HelloChild {
    fn hello(&mut self) {}
}
