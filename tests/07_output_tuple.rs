// use macros::{delegate_trait, Delegate};
//
// /// 複数のフィールド、複数のトレイトを委譲できるようにします。
// #[delegate_trait]
// trait Transformable {
//     fn position(&self) -> (usize, usize);
// }
//
//
// struct Transform {
//     x: usize,
//     y: usize,
// }
//
//
// impl Transformable for Transform {
//     fn position(&self) -> (usize, usize) {
//         (self.x, self.y)
//     }
// }
//
//
// #[derive(Delegate)]
// struct Parent {
//     #[by(Transformable)]
//     transform: Transform,
// }
//
//
// fn main() {
//     let parent = Parent {
//         transform: Transform { x: 0, y: 10 },
//     };
//
//
//     assert_eq!(parent.position(), (0, 10));
// }
