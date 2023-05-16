use auto_delegate::Delegate;

use crate::common::increment::child::IncrementChild;

#[derive(Default, Delegate)]
pub struct IncrementParent {
    #[to(Increment)]
    child: IncrementChild,
}
