use auto_delegate_impl::Delegate;

use crate::common::addr::child::AddChild;

#[derive(Delegate, Default)]
pub struct AddrParent {
    #[to(Addr)]
    child: AddChild,
}
