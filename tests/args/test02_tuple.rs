use auto_delegate::{delegate, Delegate};

#[delegate]
trait Transformable {
    fn position(&self) -> (usize, usize);

    fn move_to(&mut self, pos: (usize, usize));
}

#[derive(Default)]
struct Child {
    pos: (usize, usize),
}

impl Transformable for Child {
    fn position(&self) -> (usize, usize) {
        self.pos
    }

    fn move_to(&mut self, pos: (usize, usize)) {
        self.pos = pos;
    }
}


#[derive(Delegate, Default)]
struct Parent {
    #[to(Transformable)]
    child: Child,
}


fn main() {
    let mut parent = Parent::default();
    assert_eq!(parent.position(), (0, 0));

    parent.move_to((3, 3));
    assert_eq!(parent.position(), (3, 3));
}