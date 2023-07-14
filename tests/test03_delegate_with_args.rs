use common::addr::Addr;

use crate::common::addr::parent::AddrParent;

mod common;

fn main() {
    let parent = AddrParent::default();
    
    assert_eq!(parent.add(1, 3), 4);
}