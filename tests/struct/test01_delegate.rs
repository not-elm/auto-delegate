use auto_delegate::{delegate, Delegate};

#[delegate]
trait Adder {
    fn increment(&mut self) -> usize;
}


struct Child {
    num: usize,
}

impl Adder for Child {
    fn increment(&mut self) -> usize {
        self.num += 1;
        self.num
    }
}


#[derive(Delegate)]
struct Parent {
    #[to(Adder)]
    child: Child,
}


fn main() {
    let mut parent = Parent {
        child: Child { num: 0 },
    };
    
    assert_eq!(parent.increment(), 1);
    assert_eq!(parent.increment(), 2);
    assert_eq!(parent.increment(), 3);
}
