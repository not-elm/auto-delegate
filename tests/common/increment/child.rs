use crate::common::increment::Addr;

pub struct IncrementChild {
    num: usize,
}


impl Addr for IncrementChild {
    fn increment(&mut self) -> usize {
        self.num += 1;
        self.num
    }
}


impl Default for IncrementChild {
    fn default() -> Self {
        Self {
            num: 0
        }
    }
}


