use crate::common::addr::Addr;
use crate::common::increment::Increment;
use crate::common::multiple_field::child::Readable;
use crate::common::multiple_field::parent::MultipleParent;

mod common;


fn main() {
    let mut parent = MultipleParent::<usize>::new();

    assert_eq!(parent.add(2, 3), 5);

    assert_eq!(parent.increment(), 1);

    assert_eq!(parent.read(), &0);
}