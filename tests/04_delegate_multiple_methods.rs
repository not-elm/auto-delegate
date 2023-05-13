// use macros::{delegate_trait, Delegate};
//
// /// 複数のメソッド、返り値がないメソッドを持つトレイトを委譲できるようにします。
// #[delegate_trait]
// trait Transformable {
//     fn move_to_x(&mut self, x: usize);
//     fn move_to_y(&mut self, y: usize);
// }
//
//
// struct Child {
//     x: usize,
//     y: usize,
// }
//
//
// impl Child {
//     pub const fn new() -> Self {
//         Self { x: 0, y: 0 }
//     }
// }
//
//
// impl Transformable for Child {
//     fn move_to_x(&mut self, x: usize) {
//         self.x = x;
//     }
//
//     fn move_to_y(&mut self, y: usize) {
//         self.y = y;
//     }
// }
//
// #[derive(Delegate)]
// struct Parent {
//     #[by(Transformable)]
//     child: Child,
// }
//
//
// fn main() {
//     let mut parent = Parent {
//         child: Child::new(),
//     };
//
//     parent.move_to_x(3);
//     assert_eq!(parent.child.x, 3);
//
//     parent.move_to_y(5);
//     assert_eq!(parent.child.y, 5);
// }
