pub(crate) use crate::a;
use crate::child::HelloChild;
pub(crate) use crate::impl_delegate_hello;
a!();
impl_delegate_hello!();
pub struct HelloParent {
    child: HelloChild,
}


impl HelloParent {
    pub fn new() -> Self {
        Self {
            child: HelloChild {},
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::child::HelloChild;
    use crate::parent::HelloParent;
    use crate::transform::Hello;

    #[test]
    fn it() {
        let mut parent = HelloChild {};
        parent.hello();
    }


    #[test]
    fn it2() {
        let mut parent = HelloParent::new();
        parent.hello();
    }
}
