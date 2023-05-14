use crate::common::increment::Increment;

pub struct IncrementChild {
    num: usize,
}


impl Increment for IncrementChild {
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


