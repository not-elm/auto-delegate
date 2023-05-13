// use macros::{delegate_trait, Delegate};
//
// /// 複数のトレイトを実装したフィールドから、
// /// それぞれの処理を委譲できるようにします。
//
// #[delegate_trait]
// trait Transformable {
//     fn move_to_x(&mut self, x: usize);
//     fn move_to_y(&mut self, y: usize);
// }
//
// #[delegate_trait]
// trait Resizable {
//     fn resize(&mut self, width: usize, height: usize);
// }
//
//
// #[delegate_trait]
// trait Number {
//     fn num(&self) -> i32;
// }
//
//
// struct Child {
//     x: usize,
//     y: usize,
//     width: usize,
//     height: usize,
// }
//
//
// impl Child {
//     pub const fn new() -> Self {
//         Self {
//             x: 0,
//             y: 0,
//             width: 0,
//             height: 0,
//         }
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
//
// impl Resizable for Child {
//     fn resize(&mut self, width: usize, height: usize) {
//         self.width = width;
//         self.height = height;
//     }
// }
//
//
// impl Number for Child {
//     fn num(&self) -> i32 {
//         3
//     }
// }
//
// #[derive(Delegate)]
// struct Parent {
//     #[by(Transformable, Resizable, Number)]
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
//
//
//     parent.resize(100, 300);
//     assert_eq!(parent.child.width, 100);
//     assert_eq!(parent.child.height, 300);
//
//
//     assert_eq!(parent.num(), 3);
// }
