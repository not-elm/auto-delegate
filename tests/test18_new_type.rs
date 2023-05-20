use auto_delegate_macros::Delegate;

use crate::common::addr::child::AddChild;
use crate::common::addr::Addr;

mod common;


#[derive(Delegate, Default)]
#[to(Addr)]
struct Parent(AddChild);


fn main() {
    let parent = Parent::default();

    assert_eq!(parent.add(3, 2), 5);
}
