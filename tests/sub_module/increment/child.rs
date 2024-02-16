use crate::sub_module::increment::Increment;

#[derive(Default)]
pub struct IncrementChild {
    num: usize,
}


impl Increment for IncrementChild {
    fn increment(&mut self) -> usize {
        self.num += 1;
        self.num
    }
}





