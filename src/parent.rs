#[cfg(test)]
mod tests {
    use crate::child::HelloChild;
    use crate::transform::Hello;
    use macros::Delegate;

    #[derive(Delegate)]
    pub struct HelloParent {
        #[by(Hello)]
        child: HelloChild,
    }


    impl HelloParent {
        pub fn new() -> Self {
            Self {
                child: HelloChild {},
            }
        }
    }


    #[test]
    fn it2() {
        let mut parent = HelloParent::new();

        let num = parent.hello_ref(3);
        assert_eq!(num, 4);
    }
}
