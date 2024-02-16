use auto_delegate_impl::Delegate;

use crate::sub_module::addr::child::AddChild;

#[derive(Delegate, Default)]
pub struct AddrParent {
    #[to(Addr)]
    child: AddChild,
}
