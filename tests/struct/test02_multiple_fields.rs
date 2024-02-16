use auto_delegate_impl::{delegate, Delegate};

#[delegate]
trait Calc {
    fn calc(&self, lhs: usize, rhs: usize) -> usize;
}

#[delegate]
trait Increment {
    fn increment(&mut self) -> usize;
}

#[derive(Default)]
struct Child(usize);

impl Calc for Child {
    fn calc(&self, lhs: usize, rhs: usize) -> usize {
        lhs + rhs
    }
}

impl Increment for Child {
    fn increment(&mut self) -> usize {
        self.0 += 1;
        self.0
    }
}

#[delegate]
trait Hello {
    fn hello(&mut self);
}

#[delegate]
trait Number {
    fn num(&self) -> usize;
}

#[derive(Default)]
struct HelloChild;

impl Hello for HelloChild {
    fn hello(&mut self) {}
}

impl Number for HelloChild {
    fn num(&self) -> usize {
        3
    }
}

#[derive(Delegate, Default)]
struct Parent {
    #[to(Calc, Increment)]
    child: Child,

    #[to(Hello, Number)]
    child2: HelloChild,
}

fn main() {
    let mut parent = Parent::default();
    assert_eq!(parent.calc(2, 3), 5);
    assert_eq!(parent.increment(), 1);
    parent.hello();
    assert_eq!(parent.num(), 3);
}
