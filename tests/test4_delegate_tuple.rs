use auto_delegate_macros::{Delegate, delegate_trait};

#[delegate_trait]
trait Transformable {
    fn position(&self) -> (usize, usize);

    fn move_to(&mut self, pos: (usize, usize));
}


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


impl Default for Child {
    fn default() -> Self {
        Self {
            pos: (0, 0)
        }
    }
}


#[derive(Delegate, Default)]
struct Parent {
    #[by(Transformable)]
    child: Child,
}


fn main() {
    let mut parent = Parent::default();

    assert_eq!(parent.position(), (0, 0));

    parent.move_to((3, 3));

    assert_eq!(parent.position(), (3, 3));
}
