use macros::{delegate_trait, Delegate};

/// トレイト、引数、返り値にライフタイムを指定できるようにします。
#[delegate_trait]
trait Transformable<'a> {
    fn position_ref(&'a self) -> &'a (usize, usize);
    fn position_mut(&'a mut self) -> &'a mut (usize, usize);
}


struct Transform<'a> {
    pos: &'a mut (usize, usize),
}


impl<'a> Transformable<'a> for Transform<'a> {
    fn position_ref(&'a self) -> &'a (usize, usize) {
        &self.pos
    }

    fn position_mut(&'a mut self) -> &'a mut (usize, usize) {
        &mut self.pos
    }
}


#[derive(Delegate)]
struct Parent<'a> {
    #[by(Transformable)]
    transform: Transform<'a>,
}


impl<'a> Parent<'a> {
    pub fn new(pos: &'a mut (usize, usize)) -> Parent<'a> {
        Parent {
            transform: Transform { pos },
        }
    }
}


fn main() {
    let mut pos = (0, 0);
    let mut parent = Parent::new(&mut pos);
    assert_eq!(*parent.position_ref(), (0, 0));
    {
        let pos = parent.position_mut();
        *pos = (10, 30);
    }
    assert_eq!(pos, (10, 30));
}
