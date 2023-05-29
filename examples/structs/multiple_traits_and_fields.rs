use auto_delegate::*;

#[delegate]
trait Calc {
    fn calc(&self, x1: usize, x2: usize) -> usize;
}


#[derive(Default)]
struct CalcAdd;


impl Calc for CalcAdd {
    fn calc(&self, x1: usize, x2: usize) -> usize {
        x1 + x2
    }
}


#[delegate]
trait Movable {
    fn move_to(&mut self, pos: (usize, usize));

    fn pos(&self) -> (usize, usize);
}


#[delegate]
trait Resizable {
    fn resize(&mut self, width: usize, height: usize);

    fn size(&self) -> (usize, usize);
}


#[derive(Default)]
struct Transform2D {
    pos: (usize, usize),
    width: usize,
    height: usize
}


impl Movable for Transform2D {
    fn move_to(&mut self, pos: (usize, usize)) {
        self.pos = pos;
    }

    fn pos(&self) -> (usize, usize) {
        self.pos
    }
}


impl Resizable for Transform2D {
    fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
    }

    fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}


#[derive(Default, Delegate)]
struct Parent<T: Default + Calc> {
    #[to(Movable, Resizable)]
    transform: Transform2D,

    #[to(Calc)]
    calculator: T
}


fn main() {
    let mut parent = Parent::<CalcAdd>::default();

    assert_eq!(parent.calc(3, 2), 5);

    parent.move_to((10, 11));
    assert_eq!(parent.pos(), (10, 11));

    parent.resize(100, 120);
    assert_eq!(parent.size(), (100, 120));
}
