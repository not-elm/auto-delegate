use crate::common::addr::Addr;

pub struct AddChild {}


impl Addr for AddChild {
    fn add(&self, x1: usize, x2: usize) -> usize {
        x1 + x2
    }
}


impl Default for AddChild {
    fn default() -> Self {
        Self {}
    }
}