use crate::common::increment::Increment;
use crate::common::increment::parent::IncrementParent;

mod common;


fn main() {
    let mut parent = IncrementParent::default();

    assert_eq!(parent.increment(), 1);
    assert_eq!(parent.increment(), 2);
    assert_eq!(parent.increment(), 3);
}
