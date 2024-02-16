use crate::sub_module::increment::Increment;
use crate::sub_module::increment::parent::IncrementParent;

mod sub_module;


fn main() {
    let mut parent = IncrementParent::default();
  
    assert_eq!(parent.increment(), 1);
    assert_eq!(parent.increment(), 2);
    assert_eq!(parent.increment(), 3);
}
