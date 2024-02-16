use auto_delegate::Delegate;

use crate::sub_module::increment::child::IncrementChild;

#[derive(Default, Delegate)]
pub struct IncrementParent {
    #[to(Increment)]
    child: IncrementChild,
}
