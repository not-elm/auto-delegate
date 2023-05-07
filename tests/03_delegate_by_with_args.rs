use macros::{delegate_trait, Delegate};

/// Delegate及びby属性によって自動で委譲が実装させるようにします。
#[delegate_trait]
trait Adder {
    fn increment(&mut self, num: usize) -> usize;
}


struct Child;

impl Adder for Child {
    fn increment(&mut self, num: usize) -> usize {
        num + 1
    }
}


#[derive(Delegate)]
struct Parent {
    #[by(Adder)]
    child: Child,
}


fn main() {
    let mut parent = Parent { child: Child {} };

    assert_eq!(parent.increment(0), 1);
    assert_eq!(parent.increment(1), 2);
    assert_eq!(parent.increment(2), 3);
}
